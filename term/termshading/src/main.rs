#![allow(clippy::approx_constant)]



mod math;
mod utils;
mod renderer;
mod entities;
mod configparser;



use minifb::{Window, WindowOptions};

use crate::configparser::{parse_config, SUNPATH};
use crate::renderer::{Buffer, Renderer};
use crate::math::Vec3;
use crate::entities::{Planet, System, ViewModel};
use crate::utils::{dump, get_user_input, print_debug, sleep};



const HEIGHT: Int = 60;
const WIDTH: Int = 240;
const TAU: Float = 6.2831855;
const PI: Float = 3.1415925;
const CONFIGPATH: &str = "config.config";



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

    // tui stuff
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let sun = Planet::cons(Vec3::cons(0, 0, 0), 50.0, Some(SUNPATH), None, true);
    let mut system = System::from(sun);
    let mut viewmodel = ViewModel::new(Vec3::cons(0, 100, 0));

    parse_config(CONFIGPATH, &mut system).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        panic!();
    });

    // ansi escape to clear terminal
    print!("\x1b[2J");
    // ansi escape to make cursor-line invisible for program
    print!("\x1b[?25l");

    loop {
        let inputs = get_user_input();
        if inputs.contains(&'p') {
            break;
        }

        let mut renderer = Renderer::cons(&viewmodel, &mut buffer, &system);
        let mut debug_renderer = Renderer::cons(&viewmodel, &mut debug_buffer, &system);
        if !debug {
            renderer.buffer.clear();
            renderer.render_planets();
            renderer.render_spacerefs();
            renderer.render_orbits();
            renderer.render_rings();
            dump(renderer);
            viewmodel.react(&inputs);
            buffer.display();
        }
        else {
            debug_renderer.buffer.clear();
            debug_renderer.render_planets();
            debug_renderer.render_spacerefs();
            debug_renderer.render_orbits();
            debug_renderer.render_rings();
            dump(debug_renderer);
            viewmodel.react(&inputs);

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
