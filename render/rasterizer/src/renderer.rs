#![cfg_attr(rustfmt, rustfmt_skip)]



use crate::{Color, Float, Int, SCREENSCALE, TERMHEIGHTWIDTH};
use crate::math::{Vec2i, Vec2u, Vec3};
use crate::geometry::{Mesh, Triangle};



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
        y * self.width + x
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

pub struct Renderer<'d> {
    buffer: &'d mut Buffer,
    scanbuffer: ScanBuffer,
    mesh: &'d Mesh,
    camera: &'d Vec3,
}

impl<'d> Renderer<'d> {
    pub fn cons(
        buffer: &'d mut Buffer, mesh: &'d Mesh, camera: &'d Vec3
    ) -> Renderer<'d> {
        let scanbuffer = ScanBuffer::cons(buffer.height);
        Renderer { buffer, scanbuffer, mesh, camera }
    }

    pub fn draw_bounding_box(&mut self, color: Color) {
        self.buffer.set(0, 0, color);
        self.buffer.set(0, self.buffer.height-1, color);
        self.buffer.set(self.buffer.width-1, 0, color);
        self.buffer.set(self.buffer.width-1, self.buffer.height-1, color);
    }

    pub fn render_mesh(&mut self) {
        self.mesh.tris.iter().for_each(|tri| {
            self.render_triangle(tri);
        });
    }

    pub fn render_triangle(&mut self, tri: &'d Triangle) {
        // triangle sent
        let Triangle { mut a, mut b, mut c } = *tri;
        // vertices in screen coordinates
        a -= *self.camera;
        b -= *self.camera;
        c -= *self.camera;

        // vertices put into screen coordinates
        let mut a: Vec2i = self.view_to_screen(&a);
        let mut b: Vec2i = self.view_to_screen(&b);
        let mut c: Vec2i = self.view_to_screen(&c);

        // sort vertices with a bubble sort
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

        // determines which sides to draw
        let u: Vec2i = b - a;
        let v: Vec2i = c - a;

        if u.det(&v) >= 0 {
            self.scan_right_triangle(&a, &b, &c);
        }
        else {
            self.scan_left_triangle(&a, &b, &c);
        }

        // self.fill_scanbuffer_range(a.y, c.y, 0xffffaacc);
        self.draw_line(&a, &b, 0xff00ffff);
        self.draw_line(&a, &c, 0xff00ffff);
        self.draw_line(&c, &b, 0xff00ffff);
    }

    fn view_to_screen(&self, target: &Vec3) -> Vec2i {
        let scaley: Float = SCREENSCALE;
        let scalex: Float = SCREENSCALE * TERMHEIGHTWIDTH;
        if target.x <= 0.0 {
            return Vec2i::cons(self.buffer.get_half_width() as Int, self.buffer.get_half_height() as Int);
        }
        let scrx: Int = (target.y / target.x * scalex + self.buffer.get_half_width()) as Int;
        let scry: Int = (-target.z / target.x * scaley + self.buffer.get_half_height()) as Int;
        Vec2i::cons(scrx, scry)
    }

    fn scan_right_triangle(&mut self, a: &Vec2i, b: &Vec2i, c: &Vec2i) {
        self.scan_convert_high(a, c);
        self.scan_convert_low(a, b);
        self.scan_convert_low(b, c);
    }

    fn scan_left_triangle(&mut self, a: &Vec2i, b: &Vec2i, c: &Vec2i) {
        self.scan_convert_low(a, c);
        self.scan_convert_high(a, b);
        self.scan_convert_high(b, c);
    }

    fn scan_convert_low(&mut self, start: &Vec2i, end: &Vec2i) {
        let mut x0 = start.x;
        let x1 = end.x;
        let mut y0 = start.y;
        let y1 = end.y;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            if (y0 as usize) < self.scanbuffer.height {
                self.scanbuffer.set_low(y0 as usize, x0 as usize);
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error += dx;
                y0 += sy;
            }
        }
    }

    fn scan_convert_high(&mut self, start: &Vec2i, end: &Vec2i) {
        let mut x0 = start.x;
        let x1 = end.x;
        let mut y0 = start.y;
        let y1 = end.y;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            if (y0 as usize) < self.scanbuffer.height {
                self.scanbuffer.set_high(y0 as usize, x0 as usize);
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error += dx;
                y0 += sy;
            }
        }
    }

    fn fill_scanbuffer_range(&mut self, min: Int, max: Int, color: Color) {
        for y in max..min {
            let y = y as usize;
            if y >= self.scanbuffer.height { continue; }
            for x in self.scanbuffer.scan[y].x..=self.scanbuffer.scan[y].y {
                if self.buffer.inbounds(x, y) {
                    self.buffer.set(x, y, color);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn draw_line(&mut self, start: &Vec2i, end: &Vec2i, color: Color) {
        let mut x0 = start.x;
        let x1 = end.x;
        let mut y0 = start.y;
        let y1 = end.y;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            if self.buffer.inbounds(x0 as usize, y0 as usize) {
                self.buffer.set(x0 as usize, y0 as usize, color);
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error += dx;
                y0 += sy;
            }
        }
    }
}

#[allow(dead_code)]
pub struct EdgeTracer {
    start: Vec2i,
    end: Vec2i,
}

#[allow(dead_code)]
impl EdgeTracer {
    pub fn cons(start: Vec2i, end: Vec2i) -> EdgeTracer {
        EdgeTracer { start, end }
    }
}

struct ScanBuffer {
    height: usize,
    scan: Vec<Vec2u>,
}

impl ScanBuffer {
    fn cons(height: usize) -> ScanBuffer {
        ScanBuffer { height, scan: vec![Vec2u::cons(0, 0); height] }
    }

    fn set_low(&mut self, y: usize, value: usize) {
        {
            debug_assert!(y < self.height);
        }
        self.scan[y].x = value;
    }

    fn set_high(&mut self, y: usize, value: usize) {
        {
            debug_assert!(y < self.height);
        }
        self.scan[y].y = value;
    }
}
