#![allow(clippy::approx_constant)]



use std::{
    fs::read_to_string,
    io::{stdout, Write},
    ops::{Add, Mul},
    thread::sleep,
    time::Duration,
};



type Float = f32;
type Int = i32;



const PI: Float = 3.14159;
const TAU: Float = 2.0 * PI;

const SCREENSCALE: Float = 150.;
const TERMHEIGHTWIDTH: Float = 2.05;



#[rustfmt::skip]
fn main() {
    let mut buffer: Buffer = Buffer::cons(55, 170);
    let mut sphere: Sphere = Sphere::cons(20., Vec3::cons(0., 0., 0.),
        Some("../planet_textures/mars_map.txt"),
    );
    let mut light = Vec3::cons(-1., 0., 0.2);
    let camera = Vec3::cons(-120., 0., 0.);

    print!("\x1b[2J");
    print!("\x1b[H");
    print!("\x1b[?25l");
    loop {
        buffer.clear();
        {
            sphere.rotation.z += -0.01;
            sphere.rotation.x += 0.005;
            sphere.center.x += 0.01;
            light.rotatez(0.01);
        }

        let mut renderer = Renderer::cons(&mut buffer, &sphere, &camera, &mut light);
        renderer.render_sphere();
        renderer.render_to_screen();

        sleep(Duration::from_millis(1));
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy)]
struct Vec3 {
    x: Float, y: Float, z: Float,
}

#[allow(dead_code)]
impl Vec3 {
    fn cons(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn inner_product(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn normalize(&mut self) {
        let len: Float = self.inner_product(self).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    fn rotatex(&mut self, angle: Float) {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        self.x = x;
        self.y = y * cos - z * sin;
        self.z = y * sin + z * cos;
    }

    fn rotatey(&mut self, angle: Float) {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        self.x = x * cos + z * sin;
        self.y = y;
        self.z = -x * sin + z * cos;
    }

    fn rotatez(&mut self, angle: Float) {
        let Vec3 { x, y, z } = *self;
        let (sin, cos) = angle.sin_cos();
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
        self.z = z;
    }

    fn rotatezyx(&mut self, angles: Vec3) {
        self.rotatez(angles.z);
        self.rotatey(angles.y);
        self.rotatex(angles.x);
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3::cons(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Mul<Float> for Vec3 {
    type Output = Self;
    fn mul(self, other: Float) -> Self::Output {
        Vec3::cons(self.x * other, self.y * other, self.z * other)
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy)]
struct Color {
    red: u8, green: u8, blue: u8,
}

impl Color {
    fn cons(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    fn from_str(data: &str) -> Color {
        let mut rgb = data.split(';');
        let red   = rgb.next().unwrap().parse::<u8>().unwrap();
        let green = rgb.next().unwrap().parse::<u8>().unwrap();
        let blue  = rgb.next().unwrap().parse::<u8>().unwrap();
        Color { red, green, blue }
    }

    fn to_ansi_back(self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.red, self.green, self.blue)
    }

    fn attenuate(&mut self, lighting: Float) {
        self.red   = ((self.red as Float)   * lighting) as u8;
        self.green = ((self.green as Float) * lighting) as u8;
        self.blue  = ((self.blue as Float)  * lighting) as u8;
    }

    fn is_black(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }
}

#[rustfmt::skip]
struct Buffer {
    height: usize, width: usize,
    pixels: Vec<Color>,
    depth: Vec<Float>,
}

impl Buffer {
    #[rustfmt::skip]
    fn cons(height: usize, width: usize) -> Buffer {
        Buffer {
            height, width,
            pixels: vec![Color::cons(0, 0, 0); width * height], depth: vec![1E9; width * height],
        }
    }

    fn set(&mut self, x: usize, y: usize, color: Color, depth: Float) {
        {
            debug_assert!(self.inbounds(x, y));
        }
        let idx = self.idx(x, y);
        self.pixels[idx] = color;
        self.depth[idx] = depth;
    }

    fn get_depth(&self, x: usize, y: usize) -> Float {
        self.depth[self.idx(x, y)]
    }

    fn get_half_height(&self) -> Float {
        (self.height / 2) as Float
    }

    fn get_half_width(&self) -> Float {
        (self.width / 2) as Float
    }

    fn clear(&mut self) {
        self.pixels.fill(Color::cons(0, 0, 0));
        self.depth.fill(1E9);
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

#[rustfmt::skip]
struct Texture {
    height: usize, width: usize,
    texture: Vec<Color>,
}

impl Texture {
    #[rustfmt::skip]
    fn from_file(path: &str) -> Texture {
        let contents = read_to_string(path).unwrap();

        let mut height = 0;
        let mut width = 0;
        let mut texture: Vec<Color> = Vec::new();
        contents.lines().for_each(|line| {
            let mut linewidth = 0;
            line.split_whitespace().for_each(|col| {
                let mut color = Color::from_str(col);
                color.attenuate(1.1);
                texture.push(color);
                linewidth += 1;
            });
            height += 1;
            width = width.max(linewidth);
        });

        Texture { height, width, texture }
    }

    fn get_at(&self, xfrac: Float, yfrac: Float) -> Color {
        let texx = self.width-1 - ((xfrac * self.width as Float) as usize);
        let texy = (yfrac * self.height as Float) as usize;
        self.texture[texy * self.width + texx]
    }
}

struct Sphere {
    radius: Float,
    center: Vec3,
    rotation: Vec3,
    texture: Option<Texture>,
}

impl Sphere {
    fn cons(radius: Float, center: Vec3, texpath: Option<&str>) -> Sphere {
        Sphere {
            radius,
            center,
            rotation: Vec3::cons(0., 0., 0.),
            texture: texpath.map(Texture::from_file),
        }
    }
}

struct Renderer<'d> {
    buffer: &'d mut Buffer,
    sphere: &'d Sphere,
    camera: &'d Vec3,
    light: &'d Vec3,
    fbuffer: String,
}

impl<'d> Renderer<'d> {
    #[rustfmt::skip]
    fn cons(
        buffer: &'d mut Buffer,sphere: &'d Sphere,camera: &'d Vec3,light: &'d mut Vec3
    ) -> Renderer<'d> {
        light.normalize();
        Renderer { buffer, sphere, camera, light, fbuffer: String::new() }
    }

    #[rustfmt::skip]
    fn render_sphere(&mut self) {
        let delta = 0.007;
        let (thetadelta, phidelta) = (delta, delta * 2.);
        let thetastep = (TAU / thetadelta) as Int;
        let phistep = (PI / phidelta) as Int;

        let scaley = SCREENSCALE;
        let scalex = SCREENSCALE * TERMHEIGHTWIDTH;

        for thetamul in 0..thetastep {
            let theta = thetamul as Float * thetadelta;
            for phimul in 0..phistep {
                let phi = phimul as Float * phidelta;

                let (sint, cost) = theta.sin_cos();
                let (sinp, cosp) = phi.sin_cos();

                let spherex = self.sphere.radius * cost * sinp;
                let spherey = self.sphere.radius * sint * sinp;
                let spherez = self.sphere.radius * cosp;
                let mut world = Vec3::cons(spherex, spherey, spherez);
                world.rotatex(self.sphere.rotation.x);
                world.rotatez(self.sphere.rotation.z);

                let mut normal = world;
                normal.normalize();

                world = world + self.sphere.center;

                let view = world + *self.camera * -1.;

                let scrx = (view.y / view.x * scalex + self.buffer.get_half_width()) as usize;
                let scry = (-view.z / view.x * scaley + self.buffer.get_half_height()) as usize;

                if self.buffer.inbounds(scrx, scry) && self.buffer.get_depth(scrx, scry) >= world.x {
                    let mut color = Color::cons(0, 255, 255);
                    if let Some(texture) = &self.sphere.texture {
                        color = texture.get_at(theta / TAU, phi / PI);
                    }
                    let lighting = self.calc_lighting(&normal);
                    color.attenuate(lighting);

                    self.buffer.set(scrx, scry, color, world.x);
                }
            }
        }
    }

    fn calc_lighting(&self, normal: &Vec3) -> Float {
        self.light.inner_product(normal).max(0.5)
    }

    #[rustfmt::skip]
    fn render_to_screen(&mut self) {
        self.fbuffer.clear();
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
        print!("{}", self.fbuffer);
        stdout().flush().unwrap();
    }
}
