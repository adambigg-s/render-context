#![cfg_attr(rustfmt, rustfmt_skip)]



use crate::{Float, Int};
use crate::render_utils::{Buffer, Camera, Color};
use crate::math::{Vec2i, Vec3f};
use crate::geometry::{Barycentric, Mesh, Tri, Vert};



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

    #[allow(dead_code)]
    pub fn render_wireframe(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            if let Some(triangle) = self.prep_triangle(tri, self.mesh.rotation) {
                self.draw_line(triangle.a.pos, triangle.b.pos);
                self.draw_line(triangle.a.pos, triangle.c.pos);
                self.draw_line(triangle.c.pos, triangle.b.pos);
            }
        })
    }

    pub fn render_triangle(&mut self, tri: &Tri, rotation: Vec3f) {
        if let Some(triangle) = self.prep_triangle(tri, rotation) {
            let (a, b, c) = (triangle.a.pos, triangle.b.pos, triangle.c.pos);
            if triangle.long_left() {
                self.trace_and_fill(&triangle, a, c, a, b);
                self.trace_and_fill(&triangle, c, a, c, b);
            }
            else {
                self.trace_and_fill(&triangle, a, b, a, c);
                self.trace_and_fill(&triangle, c, b, c, a);
            }
        }
    }

    fn trace_and_fill(&mut self, triangle: &Tri, e1s: Vec3f, e1e: Vec3f, e2s: Vec3f, e2e: Vec3f) {
        let mut e1 = EdgeTracer::cons(e1s, e1e);
        let mut e2 = EdgeTracer::cons(e2s, e2e);
        while let (Some(p1), Some(p2)) = (e1.step_constant(), e2.step_constant()) {
            self.fill_edge_trace(&p1, &p2, triangle);
        }
    }

    fn prep_triangle(&mut self, tri: &Tri, rotation: Vec3f) -> Option<Tri> {
        let mut triangle: Tri = *tri;
        triangle.rotatezyx(rotation);

        let norm = triangle.get_normal();
        if norm.x >= 0. {
            return None;
        }
        
        let Tri { mut a, mut b, mut c, .. } = triangle;
        a.pos -= self.camera.position;
        b.pos -= self.camera.position;
        c.pos -= self.camera.position;

        let mut a: Vert = self.view_to_screen(&a);
        let mut b: Vert = self.view_to_screen(&b);
        let mut c: Vert = self.view_to_screen(&c);

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
        Some(Tri::cons_verts(a, b, c))
    }

    fn fill_edge_trace(&mut self, starting: &Vec2i, ending: &Vec2i, triangle: &Tri) {
        {
            debug_assert!(starting.y == ending.y);
        }
        let y = starting.y;
        for x in starting.x..=ending.x {
            if !self.buffer.inbounds(x as usize, y as usize) { continue; }
            let barycentric = Barycentric::cons(triangle);
            let barys = barycentric.weights(x, y);
            let red = triangle.get_color_red().inner_prod(&barys);
            let green = triangle.get_color_green().inner_prod(&barys);
            let blue = triangle.get_color_blue().inner_prod(&barys);
            let color = Color::cons(red as u8, green as u8, blue as u8);
            self.buffer.set(x as usize, y as usize, color);
        }
    }

    pub fn draw_line(&mut self, p1: Vec3f, p2: Vec3f) {
        let mut edge = EdgeTracer::cons(p1, p2);
        while let Some(point) = edge.step_once() {
            self.buffer.set(point.x as usize, point.y as usize, Color::cons(0, 255, 255));
        }
    }

    fn view_to_screen(&self, target: &Vert) -> Vert {
        let scrx  = target.pos.y / target.pos.x * self.scale + self.buffer.get_half_width();
        let scry = -target.pos.z / target.pos.x * self.scale + self.buffer.get_half_height();
        Vert::cons(Vec3f::cons(scrx, scry, target.pos.z), target.color)
    }
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
        let start = Vec2i::cons(start.x as Int, start.y as Int);
        let end = Vec2i::cons(end.x as Int, end.y as Int);
        let dx: i32 = (end.x - start.x).abs();
        let dy: i32 = -(end.y - start.y).abs();
        let sx: i32 = if start.x < end.x { 1 } else { -1 };
        let sy: i32 = if start.y < end.y { 1 } else { -1 };
        let error: i32 = dx + dy;
        
        EdgeTracer { current: start, end, step: Vec2i::cons(sx, sy), deltas: Vec2i::cons(dx, dy), error }
    }

    pub fn step_once(&mut self) -> Option<Vec2i> {
        let e2: i32 = 2 * self.error;
        if e2 >= self.deltas.y {
            if self.current.x == self.end.x { return None; }
            self.error += self.deltas.y;
            self.current.x += self.step.x;
        }
        else if e2 <= self.deltas.x {
            if self.current.y == self.end.y { return None; }
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
