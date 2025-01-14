#![allow(clippy::approx_constant)]



mod math;
mod utils;
mod renderer;
mod entities;
mod configparser;



use crate::configparser::{general_config, parse_config, Config, SUNPATH};
use crate::renderer::{Buffer, Renderer};
use crate::math::Vec3;
use crate::entities::{Planet, System, ViewModel};
use crate::utils::{dump, get_user_input, print_debug, sleep};



const TAU: Float = 6.2831855;
const PI: Float = 3.1415925;
const SYSTEMCONFIG: &str = "systemconfig.config";
const CONFIG: &str = "config.config";
const FRAMEDELAY: u64 = 1000 / 60;



type Float = f32;
type Int = i32;

fn main() {
    let mut config: Config = general_config(CONFIG).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        panic!();
    });

    let mut buffer = Buffer::cons(config.height(), config.width());
    let sun = Planet::cons("sun".to_owned(), Vec3::cons(0, 0, 0), 695700.0,
        Some(SUNPATH), true, None);
    let mut system = System::from(sun);
    let mut viewmodel = ViewModel::new(Vec3::cons(0, 0, 0));

    parse_config(SYSTEMCONFIG, &mut system).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        panic!();
    });
    system.transform_mini();
    viewmodel.goto("earth", &system);
    
    // ansi escape to clear terminal
    print!("\x1b[2J");
    // ansi escape to make cursor-line invisible for program
    print!("\x1b[?25l");

    loop {
        let inputs = get_user_input();
        if inputs.contains(&'p') {
            break;
        }

        let mut renderer = Renderer::cons(&viewmodel, &mut buffer, &system, &config);
        renderer.buffer.clear();
        renderer.render_planets();
        if config.render_refs() {
            renderer.render_spacerefs();
        }
        if config.render_orbits() {
            renderer.render_orbits();
        }
        renderer.render_rings();
        dump(renderer);
        viewmodel.react(&inputs, &system, &mut config);
        buffer.display();
    
        print_debug(&viewmodel);
        sleep(FRAMEDELAY);
    }

    // debugging stuff here. planets[3] should likely be earth, but if it crashes at this point
    // it doesn't really matter anway cause last few lines
    println!("{:?}", system.planets[3].name);
    println!("{:?}", system.planets[3].loc);
    println!("{:?}", system.planets[3].rad);
    println!("{:?}", system.planets[3].lightsource);
    println!("{:?}", system.planets[3].features);
}
