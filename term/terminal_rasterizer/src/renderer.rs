


use std::{io::{stdout, Write}, mem::swap};

use crate::{geometry::Triangle, math::{Vec2i, Vec2u, Vec3}, Float, Int, SCREENSCALE, TERMHEIGHTWIDTH};



#[rustfmt::skip]
#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8, pub green: u8, pub blue: u8,
}

impl Color {
    pub fn cons(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn to_ansi_back(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.red, self.green, self.blue)
    }

    pub fn attenuate(&mut self, lighting: Float) {
        self.red = ((self.red as Float) * lighting) as u8;
        self.green = ((self.green as Float) * lighting) as u8;
        self.blue = ((self.blue as Float) * lighting) as u8;
    }

    pub fn is_black(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }
}

#[rustfmt::skip]
pub struct Buffer {
    pub height: usize, pub width: usize,
    pixels: Vec<Color>,
    depth: Vec<Float>,
}

impl Buffer {
    #[rustfmt::skip]
    pub fn cons(height: usize, width: usize) -> Buffer {
        Buffer {
            height, width,
            pixels: vec![Color::cons(0, 0, 0); width * height], depth: vec![1E9; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color, depth: Float) {
        {
            debug_assert!(self.inbounds(x, y));
        }
        let idx = self.idx(x, y);
        self.pixels[idx] = color;
        self.depth[idx] = depth;
    }

    pub fn get_depth(&self, x: usize, y: usize) -> Float {
        self.depth[self.idx(x, y)]
    }

    pub fn get_half_height(&self) -> Float {
        (self.height / 2) as Float
    }

    pub fn get_half_width(&self) -> Float {
        (self.width / 2) as Float
    }

    pub fn clear(&mut self) {
        self.pixels.fill(Color::cons(0, 0, 0));
        self.depth.fill(1E9);
    }

    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

pub struct Renderer<'d> {
    buffer: &'d mut Buffer,
    fbuffer: &'d mut String,
    scanbuffer: ScanBuffer,
    tri: &'d Triangle,
    camera: &'d Vec3,
}

impl<'d> Renderer<'d> {
    pub fn cons(
        buffer: &'d mut Buffer, fbuffer: &'d mut String, tri: &'d Triangle, camera: &'d Vec3
    ) -> Renderer<'d> {
        let scanbuffer = ScanBuffer::cons(buffer.height);
        Renderer { buffer, fbuffer, scanbuffer, tri, camera }
    }

    pub fn render_triangle(&mut self) {
        // this current tri is defined in world space
        // we need to convert it into screenspace coordinates
        // then rasterize on the screen
        let Triangle { mut a, mut b, mut c } = *self.tri;
        a -= *self.camera;
        b -= *self.camera;
        c -= *self.camera;

        let mut a = self.view_to_screen(&a);
        let mut b = self.view_to_screen(&b);
        let mut c = self.view_to_screen(&c);

        if c.y > b.y {
            (c, b) = (b, c);
        }
        if b.y > a.y {
            (a, b) = (b, a);
        }

        let u = a - c;
        let v = a - b;

        if u.det(&v) <= 0 {
            self.draw_rhs_tri(&a, &c, &b);
        }
        else {
            self.draw_lhs_tri(&a, &c, &b);
        }

        self.draw_line(&a, &b);
        self.draw_line(&a, &c);
        self.draw_line(&b, &c);

        self.fill_scan_convert();
    }

    fn fill_scan_convert(&mut self) {
        for y in 0..self.scanbuffer.height {
            let test = self.scanbuffer.scan[y];
            for x in test.0..=test.1 {
                if self.buffer.inbounds(x, y) {
                    self.buffer.set(x, y, Color::cons(255, 0, 255), 1.);
                }
            }
        }
    }

    fn draw_rhs_tri(&mut self, a: &Vec2i, b: &Vec2i, c: &Vec2i) {
        self.scan_convert_high(a, c);
        self.scan_convert_low(a, b);
        self.scan_convert_low(b, c);
    }

    fn draw_lhs_tri(&mut self, a: &Vec2i, b: &Vec2i, c: &Vec2i) {
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
            if y0 < self.scanbuffer.height as Int {
                self.scanbuffer.set_high(y0 as usize, x0 as usize);
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy;
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
            if y0 < self.scanbuffer.height as Int {
                self.scanbuffer.set_high(y0 as usize, x0 as usize);
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy;
            }
        }
    }

    fn draw_line(&mut self, start: &Vec2i, end: &Vec2i) {
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
                self.buffer.set(x0 as usize, y0 as usize, Color::cons(0, 255, 0), 1.);
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy;
            }
        }
    }

    fn view_to_screen(&self, target: &Vec3) -> Vec2i {
        let scaley = SCREENSCALE;
        let scalex = SCREENSCALE * TERMHEIGHTWIDTH;
        let scrx = (target.y / target.x * scalex + self.buffer.get_half_width()) as Int;
        let scry = (-target.z / target.x * scaley + self.buffer.get_half_height()) as Int;
        Vec2i::cons(scrx, scry)
    }

    #[rustfmt::skip]
    pub fn render_to_screen(&mut self) {
        self.fbuffer.clear();
        self.fbuffer.push_str("\x1b[2J");
        self.fbuffer.push_str("\x1b[H");
        for y in 0..self.buffer.height {
            for x in 0..self.buffer.width {
                let idx = self.buffer.idx(x, y);
                let color = self.buffer.pixels[idx];
                if !color.is_black() {
                    self.fbuffer.push_str(&color.to_ansi_back());
                    self.fbuffer.push(' ');
                }
                else {
                    self.fbuffer.push_str("\x1b[0m");
                    self.fbuffer.push(' ');
                }
            }
            self.fbuffer.push_str("\x1b[0m\n");
        }
        println!("{}", self.fbuffer);
        stdout().flush().unwrap();
    }
}

struct ScanBuffer {
    height: usize,
    scan: Vec<(usize, usize)>,
}

impl ScanBuffer {
    fn cons(height: usize) -> ScanBuffer {
        ScanBuffer { height, scan: vec![(0, 0); height] }
    }

    fn set_low(&mut self, y: usize, value: usize) {
        self.scan[y].0 = value;
    }

    fn set_high(&mut self, y: usize, value: usize) {
        self.scan[y].1 = value;
    }
}
