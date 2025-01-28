


mod math;
mod utils;



use std::io::Write;

use math::{Floatify, Vec3};



type Float = f32;
type Int = i32;

fn main() {
    let mut buffer = Buffer::cons(50, 200);
    let sphere = Sphere::cons(Vec3::cons(100, 0, 0), 10);

    loop {
        println!("\x1b[?25l");
        buffer.clear();

        buffer.set_pixel(0, 0, Color::cons(0, 255, 255));
        buffer.set_pixel(buffer.width-1, buffer.height-1, Color::cons(0, 255, 255));

        let renderer = Renderer { buffer: &mut buffer, sphere: &sphere };
        renderer.render_screen();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: Float,
}

impl Sphere {
    pub fn cons<T: Floatify>(origin: Vec3, radius: T) -> Sphere {
        Sphere { origin, radius: radius.floatify() }
    }
}

pub struct Buffer {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Vec<Option<Color>>>,
}

impl Buffer {
    pub fn cons(height: usize, width: usize) -> Buffer {
        Buffer { height, width, pixels: vec![vec![None; width]; height] }
    }

    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|vec| vec.fill(None));
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Color> {
        let ytrans = self.height-1 - y;
        self.pixels[ytrans][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, data: Color) {
        {
            debug_assert!(self.inbounds(x, y));
        }
        let ytrans = self.height-1 - y;
        self.pixels[ytrans][x] = Some(data);
    }

    pub fn half_height_width(&self) -> (Int, Int) {
        ((self.height / 2) as Int, (self.width / 2) as Int)
    }

    pub fn height(&self) -> Int {
        self.height as Int
    }

    pub fn width(&self) -> Int {
        self.width as Int
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn cons(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    fn to_ansi_back(self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.red, self.green, self.blue)
    }
}

pub struct Renderer<'d> {
    sphere: &'d Sphere,
    view: &'d mut View,
    buffer: &'d mut Buffer,
}

impl Renderer<'_> {
    pub fn render_sphere(&mut self) {
        let camdist = 20.0;
        for y in 0..self.buffer.height() {
            for x in 0..self.buffer.width() {
                let (hh, hw) = self.buffer.half_height_width();
                let xoffset = x - hw;
                let yoffset = y - hh;
            }
        }
    }

    pub fn render_screen(&self) {
        let mut output = String::new();
        output.push_str("\x1b[2J");
        output.push_str("\x1b[00H");
        (0..self.buffer.height).for_each(|y| {
            (0..self.buffer.width).for_each(|x| {
                if let Some(color) = self.buffer.get(x, y) {
                    output.push_str(&color.to_ansi_back());
                }
                output.push(' ');
                output.push_str("\x1b[0m");
            });
            output.push('\n');
        });
        print!("{}", output);
        std::io::stdout().flush().unwrap();
    }
}

pub struct View {
    position: Vec3,
    rotation: Float,
    tilt: Float,
}
