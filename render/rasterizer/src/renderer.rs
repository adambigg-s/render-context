#![cfg_attr(rustfmt, rustfmt_skip)]



use crate::{Color, Float, Int, SCREENSCALE, TERMHEIGHTWIDTH};
use crate::math::{Vec2i, Vec2u, Vec3};
use crate::geometry::{Mesh, RefFrame, Triangle};



pub struct Buffer {
    pub height: usize, pub width: usize,
    pub pixels: Vec<Color>,
}

impl Buffer {
    pub fn cons(height: usize, width: usize) -> Buffer {
        Buffer { height, width, pixels: vec![0; width * height] }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        {
            debug_assert!(self.inbounds(x, y));
        }
        let idx = self.idx(x, y);
        self.pixels[idx] = color;
    }

    pub fn get_half_height(&self) -> Float {
        (self.height / 2) as Float
    }

    pub fn get_half_width(&self) -> Float {
        (self.width / 2) as Float
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        (self.height-1 - y) * self.width + x
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

pub struct Renderer<'d> {
    buffer: &'d mut Buffer,
    mesh: &'d Mesh,
    camera: &'d Camera,
}

impl<'d> Renderer<'d> {
    pub fn cons(buffer: &'d mut Buffer, mesh: &'d Mesh, camera: &'d Camera) -> Renderer<'d> {
        Renderer { buffer, mesh, camera }
    }

    pub fn draw_bounding_box(&mut self, color: Color) {
        self.buffer.set(0, 0, color);
        self.buffer.set(0, self.buffer.height-1, color);
        self.buffer.set(self.buffer.width-1, 0, color);
        self.buffer.set(self.buffer.width-1, self.buffer.height-1, color);
    }

    #[allow(dead_code)]
    pub fn cull(&self, a: &Vec3, b: &Vec3, c: &Vec3) -> bool {
        (*a - *b).cross(&(*a - *c)).inner_prod(&Vec3::cons(1, 0, 0)) >= 0.
    }

    pub fn render_mesh(&mut self, color: Color) {
        self.mesh.tris.iter().for_each(|tri| {
            self.render_triangle(tri, self.mesh.rotation, color);
        });
    }

    pub fn render_mesh_frame(&mut self , color: Color) {
        self.mesh.tris.iter().for_each(|tri| {
            self.render_wireframe(tri, self.mesh.rotation, color);
        })
    }

    pub fn render_frame(&mut self, frame: &RefFrame) {
        let position = frame.center - self.camera.position;
        let xarm = position + Vec3::cons(frame.length, 0., 0.);
        let yarm = position + Vec3::cons(0., frame.length, 0.);
        let zarm = position + Vec3::cons(0., 0., frame.length);
        if position.x <= 0. { return; }
        self.draw_line(self.view_to_screen(&position), self.view_to_screen(&xarm), 0xffff0000);
        self.draw_line(self.view_to_screen(&position), self.view_to_screen(&yarm), 0xff00ff00);
        self.draw_line(self.view_to_screen(&position), self.view_to_screen(&zarm), 0xff0000ff);
        
    }

    pub fn render_screen_frame(&mut self) {
        let position = Vec2i::cons(10, 10);
        self.draw_line(position, position + Vec2i::cons(10, 0), 0xffff0000);
        self.draw_line(position, position + Vec2i::cons(0, 10), 0xff00ff00);
        self.buffer.set(position.x as usize, position.y as usize, 0xff0000ff);
    }

    pub fn draw_line(&mut self, start: Vec2i, end: Vec2i, color: Color) {
        let mut edge = EdgeTracer::cons(start, end);
        while let Some(point) = edge.step_general() {
            if !self.buffer.inbounds(point.x as usize, point.y as usize) { continue; }
            self.buffer.set(point.x as usize, point.y as usize, color);
        }
    }

    pub fn render_wireframe(&mut self, tri: &Triangle, rotation: Vec3, color: Color) {
        let (a, b, c) = self.prep_triangle(tri, rotation);
        self.draw_line(a, b, color);
        self.draw_line(a, c, color);
        self.draw_line(c, b, color);
    }

    pub fn render_triangle(&mut self, tri: &Triangle, rotation: Vec3, color: Color) {
        let (a, b, c) = self.prep_triangle(tri, rotation);

        if (a - b).det(&(a - c)) <= 0 {
            let mut longest = EdgeTracer::cons(a, c);
            let mut middle = EdgeTracer::cons(a, b);
            while let (Some(p1), Some(p2)) = (longest.step_constant(), middle.step_constant()) {
                self.fill_edge_trace(&p1, &p2, color);
            }
            let mut longest = EdgeTracer::cons(c, a);
            let mut middle = EdgeTracer::cons(c, b);
            while let (Some(p1), Some(p2)) = (longest.step_constant(), middle.step_constant()) {
                self.fill_edge_trace(&p1, &p2, color);
            }
        }
        else {
            let mut longest = EdgeTracer::cons(a, c);
            let mut middle = EdgeTracer::cons(a, b);
            while let (Some(p1), Some(p2)) = (middle.step_constant(), longest.step_constant()) {
                self.fill_edge_trace(&p1, &p2, color);
            }
            let mut longest = EdgeTracer::cons(c, a);
            let mut middle = EdgeTracer::cons(c, b);
            while let (Some(p1), Some(p2)) = (middle.step_constant(), longest.step_constant()) {
                self.fill_edge_trace(&p1, &p2, color);
            }
        }
    }

    fn prep_triangle(&mut self, tri: &Triangle, rotation: Vec3) -> (Vec2i, Vec2i, Vec2i) {
        let mut triangle = *tri;
        triangle.rotatezyx(rotation);
        let Triangle { mut a, mut b, mut c, .. } = triangle;
        a.pos -= self.camera.position;
        b.pos -= self.camera.position;
        c.pos -= self.camera.position;

        let mut a: Vec2i = self.view_to_screen(&a.pos);
        let mut b: Vec2i = self.view_to_screen(&b.pos);
        let mut c: Vec2i = self.view_to_screen(&c.pos);
        a.clamp_positive(self.buffer.height as Int, self.buffer.width as Int);
        b.clamp_positive(self.buffer.height as Int, self.buffer.width as Int);
        c.clamp_positive(self.buffer.height as Int, self.buffer.width as Int);

        if c.y > b.y {
            (c, b) = (b, c);
        }
        if b.y > a.y {
            (a, b) = (b, a);
        }
        if c.y > b.y {
            (c, b) = (b, c);
        }
        {
            debug_assert!(a.y >= b.y && b.y >= c.y);
        }
        (a, b, c)
    }

    pub fn fill_edge_trace(&mut self, starting: &Vec2i, ending: &Vec2i, color: Color) {
        {
            debug_assert!(starting.y == ending.y);
        }
        let y = starting.y;
        for x in starting.x..=ending.x {
            if !self.buffer.inbounds(x as usize, y as usize) { continue; }
            self.buffer.set(x as usize, y as usize, color);
        }
    }

    fn view_to_screen(&self, target: &Vec3) -> Vec2i {
        let scaley: Float = SCREENSCALE;
        let scalex: Float = SCREENSCALE * TERMHEIGHTWIDTH;
        let scrx: usize = (target.y / target.x * scalex + self.buffer.get_half_width()).floor() as usize;
        let scry: usize = (-target.z / target.x * scaley + self.buffer.get_half_height()).floor() as usize;
        let screen = Vec2u::cons(scrx, scry);
        Vec2i::fromvec2u(screen)
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
    pub fn cons(start: Vec2i, end: Vec2i) -> EdgeTracer {
        let dx: i32 = (end.x - start.x).abs();
        let dy: i32 = -(end.y - start.y).abs();
        let sx: i32 = if start.x < end.x { 1 } else { -1 };
        let sy: i32 = if start.y < end.y { 1 } else { -1 };
        let error: i32 = dx + dy;
        
        EdgeTracer { current: start, end, step: Vec2i::cons(sx, sy), deltas: Vec2i::cons(dx, dy), error }
    }

    pub fn step_constant(&mut self) -> Option<Vec2i> {
        let startingy = self.current.y;
        while self.current.y == startingy {
            let e2: i32 = 2 * self.error;
            if e2 >= self.deltas.y {
                if self.current.x == self.end.x { return None; }
                self.error += self.deltas.y;
                self.current.x += self.step.x;
            }
            if e2 <= self.deltas.x {
                if self.current.y == self.end.y { return None; }
                self.error += self.deltas.x;
                self.current.y += self.step.y
            }
        }
        Some(self.current)
    }

    pub fn step_general(&mut self) -> Option<Vec2i> {
        let e2: i32 = 2 * self.error;
        if e2 >= self.deltas.y {
            if self.current.x == self.end.x { return None; }
            self.error += self.deltas.y;
            self.current.x += self.step.x;
        }
        if e2 <= self.deltas.x {
            if self.current.y == self.end.y { return None; }
            self.error += self.deltas.x;
            self.current.y += self.step.y
        }
        Some(self.current)
    }
}

#[allow(dead_code)]
pub struct Camera {
    position: Vec3,
    rotation: Vec3,
}

impl Camera {
    pub fn cons(position: Vec3) -> Camera {
        Camera { position, rotation: Vec3::cons(0, 0, 0) }
    }
}
