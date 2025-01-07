#![allow(clippy::approx_constant)]



mod math;
mod utils;
mod renderer;
mod entities;



use crate::renderer::{Buffer, Renderer, Textures};
use crate::math::Vec3;
use crate::entities::{Sphere, System, ViewModel};
use crate::utils::{get_user_input, print_debug, sleep};



const HEIGHT: Int = 60;
const WIDTH: Int = 220;
const TAU: Float = 6.2831855;
const PI: Float = 3.1415925;
const ASCIIGRAD: &str = ".,:;+*?%#@";
const LIGHT: [Float; 3] = [-1.0, -1.3, 0.2];



type Float = f32;
type Int = i32;

fn main() {
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let sphere = Sphere::cons(Vec3::cons(50.0, 0.0, 0.0), 10.0);
    let textures = Textures::new();
    let mut viewmodel = ViewModel::new();
    let mut system = System::from(sphere);
    system.add_sphere(Sphere::cons(Vec3::cons(100.0, 0.0, 0.0), 2.0));

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
        let mut renderer = Renderer::cons(&viewmodel, &mut buffer, &system, &textures);
        renderer.render_spheres();
        buffer.display();
        viewmodel.react(&inputs);

        print_debug(&viewmodel);
        sleep(5);
    }
}
