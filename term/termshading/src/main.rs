#![allow(clippy::approx_constant)]



mod math;
mod utils;
mod renderer;



use std::io::{stdout, Write};

use crate::{math::Vec3, renderer::Renderer, utils::{get_user_input, print_debug, sleep}};



const HEIGHT: Int = 60;
const WIDTH: Int = 220;
const TAU: Float = 6.2831855;
const PI: Float = 3.1415925;
const ASCIIGRAD: &str = ".,:;+**?%%#@@";
const LIGHT: [Float; 3] = [-1.0, -1.3, 0.2];



type Float = f32;
type Int = i32;

fn main() {
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let sphere = Sphere::cons(Vec3::cons(50.0, 0.0, 0.0), 10.0);
    let mut viewmodel = ViewModel::new();
    let mut system = System::from(sphere);
    system.add(Sphere::cons(Vec3::cons(100.0, 0.0, 0.0), 30.0));

    // ansi escape to clear terminal
    print!("\x1b[2J");
    // ansi escape to make cursor-line invisible for program
    print!("\x1b[?25l");

    loop {
        let inputs = get_user_input();
        if inputs.contains(&'p') {
            break;
        }

        buffer.clear();
        let mut renderer = Renderer::cons(&viewmodel, &mut buffer, &system);
        renderer.render_spheres();
        buffer.display();
        viewmodel.react(&inputs);

        print_debug(&viewmodel);
        sleep(10);
    }
}

struct ViewModel {
    pos: Vec3,
    rot: Float,
    tilt: Float,
}

impl ViewModel {
    fn new() -> ViewModel {
        ViewModel { pos: Vec3::cons(0.0, 0.0, 0.0), rot: 0.0, tilt: 0.0 }
    }

    fn react(&mut self, inputs: &[char]) {
        for input in inputs {
            match input {
                'w' => self.translate(Vec3::cons(1.0, 0.0, 0.0)),
                's' => self.translate(Vec3::cons(-1.0, 0.0, 0.0)),
                'a' => self.translate(Vec3::cons(0.0, -1.0, 0.0)),
                'd' => self.translate(Vec3::cons(0.0, 1.0, 0.0)),
                'r' => self.translate(Vec3::cons(0.0, 0.0, 1.0)),
                'f' => self.translate(Vec3::cons(0.0, 0.0, -1.0)),
                'q' => self.rotate(-1.0),
                'e' => self.rotate(1.0),
                't' => self.tilt(1.0),
                'g' => self.tilt(-1.0),
                _ => {}
            }
        }
    }

    fn translate(&mut self, dir: Vec3) {
        let mut transdir = dir;
        transdir.rotatez(self.rot);
        self.pos = self.pos + transdir;
    }

    fn rotate(&mut self, dir: Float) {
        let speed = PI / 100.0;
        self.rot += dir * speed;
        if self.rot < 0.0 {
            self.rot += TAU;
        }
        else if self.rot > TAU {
            self.rot -= TAU;
        }
    }

    fn tilt(&mut self, dir: Float) {
        let speed = PI / 100.0;
        self.tilt += dir * speed;
        self.tilt = self.tilt.clamp(-PI / 2.0, PI / 2.0);
    }
}

pub struct Sphere {
    pub loc: Vec3,
    pub rad: Float,
}

impl Sphere {
    pub fn cons(loc: Vec3, rad: Float) -> Sphere {
        Sphere { loc, rad }
    }
}

pub struct System {
    pub spheres: Vec<Sphere>,
}

impl System {
    pub fn from(sphere: Sphere) -> System {
        System { spheres: vec![sphere] }
    }

    pub fn add(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

struct Buffer {
    height: Int, width: Int,
    visual: Vec<char>,
    color: Vec<bool>,
    depth: Vec<Float>,
}

impl Buffer {
    fn cons(height: Int, width: Int) -> Buffer {
        let (wi, he) = (width as usize, height as usize);
        debug_assert!(wi < 500 && he < 500);
        Buffer {
            height, width,
            visual: vec![' '; wi * he],
            color: vec![false; wi * he],
            depth: vec![0.0; wi * he],
        }
    }

    fn inboundsdex(&self, x: Int, y: Int) -> Option<usize> {
        let (height, width) = (self.height as usize, self.width as usize);
        let (x, y) = (x as usize, y as usize);
        if x < width && y < height {
            let ytrans = height-1 - y;
            Some(ytrans * width + x)
        }
        else {
            None
        }
    }

    fn display(&self) {
        let mut string = String::new();
        self.visual.iter().enumerate().for_each(|(idx, ele)| {
            if idx % self.width as usize != 0 {
                string.push(*ele);
            }
            else {
                string.push('\n')
            }
        });
        print!("\x1b[H");
        println!("{}", string);
        stdout().flush().unwrap();
    }

    const fn halfheight(&self) -> Int {
        self.height / 2
    }

    const fn halfwidth(&self) -> Int {
        self.width / 2
    }

    fn clear(&mut self) {
        self.visual.fill(' ');
        self.color.fill(false);
        self.depth.fill(0.0);
    }
}

