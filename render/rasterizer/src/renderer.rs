#![cfg_attr(rustfmt, rustfmt_skip)]



use crate::render_utils::{Buffer, Camera, Color};
use crate::{Float, Int};
use crate::math::{Vec2i, Vec3f};
use crate::geometry::{Mesh, Trif,Vertf};



#[allow(dead_code)]
pub struct Renderer<'d> {
    buffer: &'d mut Buffer,
    mesh: &'d Mesh,
    camera: &'d Camera,
    lighting_vec: Vec3f,
    scale: Float,
}

impl<'d> Renderer<'d> {
    pub fn cons(buffer: &'d mut Buffer, mesh: &'d Mesh, camera: &'d Camera, fov: Float) -> Renderer<'d> {
        let mut lighting_vec = Vec3f::cons(-6, 2, -2);
        lighting_vec.normalize();
        let scale = buffer.get_half_width() / (fov / 2.).tan();
        Renderer { buffer, mesh, camera, lighting_vec, scale }
    }

    pub fn render_mesh(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            self.render_triangle(tri, self.mesh.rotation);
        });
    }

    pub fn render_triangle(&mut self, tri: &Trif, rotation: Vec3f) {
        if let Some(triangle) = self.prep_triangle(tri, rotation) {
            let a = triangle.a.pos;
            let b = triangle.b.pos;
            let c = triangle.c.pos;
            if triangle.long_left() {
                let mut longest = EdgeTracer::cons(a, c);
                let mut middle = EdgeTracer::cons(a, b);
                while let (Some(p1), Some(p2)) = (longest.step_constant(), middle.step_constant()) {
                    self.fill_edge_trace(&p1, &p2, &triangle);
                }
                let mut longest = EdgeTracer::cons(c, a);
                let mut middle = EdgeTracer::cons(c, b);
                while let (Some(p1), Some(p2)) = (longest.step_constant(), middle.step_constant()) {
                    self.fill_edge_trace(&p1, &p2, &triangle);
                }
            }
            else {
                let mut longest = EdgeTracer::cons(a, c);
                let mut middle = EdgeTracer::cons(a, b);
                while let (Some(p1), Some(p2)) = (middle.step_constant(), longest.step_constant()) {
                    self.fill_edge_trace(&p1, &p2, &triangle);
                }
                let mut longest = EdgeTracer::cons(c, a);
                let mut middle = EdgeTracer::cons(c, b);
                while let (Some(p1), Some(p2)) = (middle.step_constant(), longest.step_constant()) {
                    self.fill_edge_trace(&p1, &p2, &triangle);
                }
            }
        }
    }

    fn prep_triangle(&mut self, tri: &Trif, rotation: Vec3f) -> Option<Trif> {
        let mut triangle: Trif = *tri;
        triangle.rotatezyx(rotation);

        let norm = triangle.get_normal();
        if norm.x >= 0. {
            return None;
        }
        
        let Trif { mut a, mut b, mut c, .. } = triangle;
        a.pos -= self.camera.position;
        b.pos -= self.camera.position;
        c.pos -= self.camera.position;

        let mut a: Vertf = self.view_to_screen(&a);
        let mut b: Vertf = self.view_to_screen(&b);
        let mut c: Vertf = self.view_to_screen(&c);

        if c.pos.y > b.pos.y {
            (c, b) = (b, c);
        }
        if b.pos.y > a.pos.y {
            (a, b) = (b, a);
        }
        if c.pos.y > b.pos.y {
            (c, b) = (b, c);
        }
        {
            debug_assert!(a.pos.y >= b.pos.y && b.pos.y >= c.pos.y);
        }
        Some(Trif::cons_verts(a, b, c))
    }

    fn fill_edge_trace(&mut self, starting: &Vec2i, ending: &Vec2i, triangle: &Trif) {
        {
            debug_assert!(starting.y == ending.y);
        }
        let y = starting.y;
        for x in starting.x..=ending.x {
            if !self.buffer.inbounds(x as usize, y as usize) { continue; }
            let barys = get_barys(triangle, x, y);
            let mut red = triangle.a.color.red as Float * barys.x;
            red += triangle.b.color.red as Float * barys.y;
            red += triangle.c.color.red as Float * barys.z;
            let mut green = triangle.a.color.green as Float * barys.x;
            green += triangle.b.color.green as Float * barys.y;
            green += triangle.c.color.green as Float * barys.z;
            let mut blue = triangle.a.color.blue as Float * barys.x;
            blue += triangle.b.color.blue as Float * barys.y;
            blue += triangle.c.color.blue as Float * barys.z;
            let color = Color::cons(red as u8, green as u8, blue as u8);
            self.buffer.set(x as usize, y as usize, color);
        }
    }

    fn view_to_screen(&self, target: &Vertf) -> Vertf {
        let scrx  = (target.pos.y / target.pos.x * self.scale
            + self.buffer.get_half_width()).ceil();
        let scry = (-target.pos.z / target.pos.x * self.scale
            + self.buffer.get_half_height()).ceil();
        Vertf::cons(Vec3f::cons(scrx, scry, target.pos.z), target.color)
    }
}

pub fn get_barys(triangle: &Trif, x: Int, y: Int) -> Vec3f {
    let x = x as Float;
    let y = y as Float;
    let a = triangle.a.pos;
    let b = triangle.b.pos;
    let c = triangle.c.pos;
    let den = ((b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y)) as Float;
    let w1 = (((b.y - c.y) * (x - c.x) + (c.x - b.x) * (y - c.y)) as Float) / den;
    let w2 = (((c.y - a.y) * (x - c.x) + (a.x - c.x) * (y - c.y)) as Float) / den;
    let w3 = 1. - w1 - w2;
    Vec3f::cons(w1, w2, w3)
}

pub struct EdgeTracer {
    current: Vec2i,
    end: Vec2i,
    step: Vec2i,
    deltas: Vec2i,
    error: Int,
}

impl EdgeTracer {
    pub fn cons(start: Vec3f, end: Vec3f) -> EdgeTracer {
        let start = Vec2i::cons(start.x.floor() as Int, start.y.floor() as Int);
        let end = Vec2i::cons(end.x.floor() as Int, end.y.floor() as Int);
        let dx: i32 = (end.x - start.x).abs();
        let dy: i32 = -(end.y - start.y).abs();
        let sx: i32 = if start.x < end.x { 1 } else { -1 };
        let sy: i32 = if start.y < end.y { 1 } else { -1 };
        let error: i32 = dx + dy;
        
        EdgeTracer { current: start, end, step: Vec2i::cons(sx, sy), deltas: Vec2i::cons(dx, dy), error }
    }

    pub fn step_once(&mut self) -> Option<Vec2i> {
        let e2: i32 = 2 * self.error;
        if self.current.x == self.end.x && self.current.y == self.end.y { return None; }
        if e2 >= self.deltas.y {
            self.error += self.deltas.y;
            self.current.x += self.step.x;
        }
        if e2 <= self.deltas.x {
            self.error += self.deltas.x;
            self.current.y += self.step.y
        }
        Some(self.current)
    }

    pub fn step_constant(&mut self) -> Option<Vec2i> {
        let startingy = self.current.y;
        while self.current.y == startingy {
            self.step_once()?;
        }
        Some(self.current)
    }
}
