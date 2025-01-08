#![allow(clippy::approx_constant)]



mod math;
mod utils;
mod renderer;
mod entities;



use crate::renderer::{Buffer, Color, Renderer, TextureGlobal};
use crate::math::Vec3;
use crate::entities::{Sphere, System, ViewModel};
use crate::utils::{get_user_input, print_debug, sleep};



const HEIGHT: Int = 60;
const WIDTH: Int = 220;
const TAU: Float = 6.2831855;
const PI: Float = 3.1415925;
const ASCIIGRAD: &str = ".,:;+*?%#@";
const SUNPATH: &str = "../planet_textures/sun_map.txt";
const EARTHPATH: &str = "../planet_textures/earth_map.txt";
const MOONPATH: &str = "../planet_textures/moon_map.txt";
const MARSPATH: &str = "../planet_textures/mars_map.txt";
const JUPITERPATH: &str = "../planet_textures/jupiter_map.txt";



type Float = f32;
type Int = i32;

fn main() {
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let sphere = Sphere::cons(Vec3::cons(50.0, 0.0, 0.0), 20.0,
        Some(EARTHPATH), Color::cons(0, 0, 0), false);
    let mut viewmodel = ViewModel::new();
    let mut system = System::from(sphere);
    system.add_sphere(Sphere::cons(Vec3::cons(50.0, 65.0, -14.0), 6.0,
        Some(MOONPATH), Color::cons(0, 0, 0), false));
    system.add_sphere(Sphere::cons(Vec3::cons(-10.0, 30.0, 0.0), 15.0,
        Some(JUPITERPATH), Color::cons(255, 246, 84), false));
    system.add_sphere(Sphere::cons(Vec3::cons(50.0, 40.0, -10.0), 10.0,
        Some(MARSPATH), Color::cons(0, 0, 0), false));
    system.add_sphere(Sphere::cons(Vec3::cons(15.0, 30.0, 0.0), 4.0,
        Some(SUNPATH), Color::cons(255, 246, 84), true));
    let textures = TextureGlobal::new(&system);

    // ansi escape to clear terminal
    print!("\x1b[2J");
    // ansi escape to make cursor-line invisible for program
    print!("\x1b[?25l");

    loop {
        let inputs = get_user_input();
        if inputs.contains(&'p') {
            break;
        }

        let mut renderer = Renderer::cons(&viewmodel, &mut buffer, &system, &textures);
        renderer.buffer.clear();
        renderer.render_spheres();
        buffer.display();
        viewmodel.react(&inputs);

        print_debug(&viewmodel);
        sleep(1);
    }
}
