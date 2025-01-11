#![allow(clippy::approx_constant)]



mod math;
mod utils;
mod renderer;
mod entities;
mod configparser;



use minifb::{Window, WindowOptions};

use crate::configparser::{general_config, parse_config, Config, SUNPATH};
use crate::renderer::{Buffer, Renderer};
use crate::math::Vec3;
use crate::entities::{Planet, System, ViewModel};
use crate::utils::{dump, get_user_input, print_debug, sleep};



const TAU: Float = 6.2831855;
const PI: Float = 3.1415925;
const SYSTEMCONFIG: &str = "systemconfig.config";
const CONFIG: &str = "config.config";



type Float = f32;
type Int = i32;

fn main() {
    // decides whether to open gui or tui
    let mut debug: bool = false;
    let envs: Vec<String> = std::env::args().collect();
    if envs.get(1).is_some() { debug = true; }
    let mut window: Option<Window> = None;
    if debug {
        window = Some(Window::new(
            "debug buffer",
            400, 400, WindowOptions { scale: minifb::Scale::X2, ..Default::default() }
        ).unwrap());
    }
    let mut debug_buffer = Buffer::cons(400, 400);

    let mut config: Config = general_config(CONFIG).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        panic!();
    });

    // tui stuff
    let mut buffer = Buffer::cons(config.height(), config.width());
    let sun = Planet::cons("sun".to_owned(), Vec3::cons(0, 0, 0), 100.0,
        Some(SUNPATH), true, None);
    let mut system = System::from(sun);
    let mut viewmodel = ViewModel::new(Vec3::cons(0, 0, 0));

    parse_config(SYSTEMCONFIG, &mut system).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        panic!();
    });
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
        let mut debug_renderer = Renderer::cons(&viewmodel, &mut debug_buffer, &system, &config);
        if !debug {
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
        }
        else {
            debug_renderer.buffer.clear();
            debug_renderer.render_planets();
            debug_renderer.render_spacerefs();
            debug_renderer.render_orbits();
            debug_renderer.render_rings();
            dump(debug_renderer);
            viewmodel.react(&inputs, &system, &mut config);

            if let Some(ref mut window) = window {
                window.update_with_buffer(
                    &debug_buffer.debug(),
                    debug_buffer.width as usize,
                    debug_buffer.height as usize
                ).unwrap();
            }
        }

        print_debug(&viewmodel);
        sleep(1);
    }
}
