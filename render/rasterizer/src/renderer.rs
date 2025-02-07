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
        let mut lighting_vec = Vec3f::cons(-3, 1, 4);
        lighting_vec.normalize();
        let scale = buffer.get_half_width() / (fov / 2.).to_degrees().tan();
        Renderer { buffer, mesh, camera, lighting_vec, scale }
    }

    pub fn render_mesh(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            self.render_triangle(tri);
        });
    }

    #[allow(dead_code)]
    pub fn render_wireframe(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            if let Some((triangle, lighting)) = self.prep_triangle(tri) {
                let mut color = Color::cons(0, 255, 255);
                color.attenuate(lighting);
                self.draw_line(triangle.a.pos, triangle.b.pos, color,);
                self.draw_line(triangle.a.pos, triangle.c.pos, color,);
                self.draw_line(triangle.c.pos, triangle.b.pos, color,);
            }
        })
    }

    pub fn render_triangle(&mut self, tri: &Tri) {
        if let Some((triangle, lighting)) = self.prep_triangle(tri) {
            let (a, b, c) = (triangle.a.pos, triangle.b.pos, triangle.c.pos);
            if triangle.lumped_left() {
                self.trace_and_fill(&triangle, a, c, a, b, lighting);
                self.trace_and_fill(&triangle, c, a, c, b, lighting);
            }
            else {
                self.trace_and_fill(&triangle, a, b, a, c, lighting);
                self.trace_and_fill(&triangle, c, b, c, a, lighting);
            }
        }
    }

    fn trace_and_fill(&mut self, triangle: &Tri, e1s: Vec3f, e1e: Vec3f, e2s: Vec3f, e2e: Vec3f, lighting: Float) {
        let mut e1 = EdgeTracer::cons(e1s, e1e);
        let mut e2 = EdgeTracer::cons(e2s, e2e);
        let barycentric = Barycentric::cons(triangle);
        while let (Some(p1), Some(p2)) = (e1.step_constant(), e2.step_constant()) {
            self.fill_edge_trace(&p1, &p2, triangle, lighting, &barycentric);
        }
    }

    fn prep_triangle(&mut self, tri: &Tri) -> Option<(Tri, Float)> {
        let mut triangle: Tri = *tri;
        triangle.rotatezyx(self.mesh.rotation);
        triangle.translate(-self.camera.position);
        triangle.translate(self.mesh.center);
        triangle.rotatezyx(-self.camera.rotation);

        let norm = triangle.get_normal();
        let lighting = self.lighting_vec.inner_prod(&norm).max(0.05);
        if norm.x > 0.15 {
            return None;
        }
        
        let Tri { a, b, c } = triangle;
        
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
        Some((Tri::cons_verts(a, b, c), lighting))
    }

    fn fill_edge_trace(
        &mut self, starting: &Vec2i, ending: &Vec2i, triangle: &Tri, lighting: Float, bary: &Barycentric)
    {
        {
            debug_assert!(starting.y == ending.y);
        }
        let y = starting.y;
        for x in starting.x..ending.x {
            if !self.buffer.inbounds(x as usize, y as usize) { return; }
            let mut color = Color::cons(222, 0, 0);
            let barys = bary.weights(x, y);
            let xtex = triangle.interpolate_tex_u(barys);
            let ytex = triangle.interpolate_tex_v(barys);
            if let Some(texture) = &self.mesh.texture {
                color = texture.get_texture(xtex, ytex);
            }
            color.attenuate(lighting);
            let depth = triangle.interpolate_depth_nonlinear(barys);
            self.buffer.set(x as usize, y as usize, color, depth);
        }
    }

    pub fn draw_line(&mut self, p1: Vec3f, p2: Vec3f, color: Color) {
        let mut edge = EdgeTracer::cons(p1, p2);
        while let Some(point) = edge.step_once() {
            if !self.buffer.inbounds(point.x as usize, point.y as usize) { return; }
            self.buffer.set(point.x as usize, point.y as usize, color, 1.);
        }
    }

    fn view_to_screen(&self, target: &Vert) -> Vert {
        let scrx = target.pos.y / target.pos.x * self.scale + self.buffer.get_half_width();
        let scry = -target.pos.z / target.pos.x * self.scale + self.buffer.get_half_height();
        Vert::cons(Vec3f::cons(scrx, scry, target.pos.x), target.color, target.texpos)
    }
}



pub struct EdgeTracer {
    current: Vec2i,
    target: Vec2i,
    steps: Vec2i,
    deltas: Vec2i,
    error: Int,
}

impl EdgeTracer {
    pub fn cons(start: Vec3f, end: Vec3f) -> EdgeTracer {
        let current = Vec2i::cons(start.x as Int, start.y as Int);
        let target = Vec2i::cons(end.x as Int, end.y as Int);
        let dx: i32 = (target.x - current.x).abs();
        let dy: i32 = -(target.y - current.y).abs();
        let int_step_x: i32 = if current.x < target.x { 1 } else { -1 };
        let int_step_y: i32 = if current.y < target.y { 1 } else { -1 };
        let error: i32 = dx + dy;
        
        EdgeTracer { current, target, steps: Vec2i::cons(int_step_x, int_step_y), deltas: Vec2i::cons(dx, dy), error }
    }

    pub fn step_once(&mut self) -> Option<Vec2i> {
        let twice_error: i32 = 2 * self.error;
        if twice_error >= self.deltas.y {
            if self.current.x == self.target.x { return None; }
            self.error += self.deltas.y;
            self.current.x += self.steps.x;
        }
        else if twice_error <= self.deltas.x {
            if self.current.y == self.target.y { return None; }
            self.error += self.deltas.x;
            self.current.y += self.steps.y
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
