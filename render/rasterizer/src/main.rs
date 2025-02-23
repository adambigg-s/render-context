#![allow(clippy::approx_constant)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]



mod geometry;
mod math;
mod render_utils;
mod renderer;
mod texture;
mod utils;

use std::time::Instant;

use minifb::{Key, Scale};

use geometry::RefFrame;
use math::Vec3f;
use render_utils::{Buffer, Camera, Color};
use renderer::Renderer;
use utils::{handle_camera_input, handle_mutation_input, handle_renderer_input, make_mesh, make_window};



type Float = f32;
type Int = i32;



const PI: Float = 3.141592;
const TAU: Float = 2. * PI;



const FOV: Float = 90.;
const FPS: usize = 120;
const BACKGROUND: u32 = 0xffbbbbbb;
const RESMOD: usize = 2;
const HEIGHT: usize = 1080 / RESMOD;
const WIDTH: usize = 1920 / RESMOD;



#[allow(unused_variables, unused_mut)]
fn main() {
    {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut window = make_window(&buffer, FPS, Scale::X2);
    let mut mesh = make_mesh();
    let mut camera = Camera::cons(Vec3f::cons(-100, 0, 0));
    let mut mouse = None;
    let frame = RefFrame::cons(Vec3f::cons(0, 0, 0), 80.);

    while !window.is_key_down(Key::Escape) && !window.is_key_down(Key::C) {
        let framestart = Instant::now();
        buffer.clear();

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera, FOV);
        renderer.render_refframe(&frame);
        handle_renderer_input(&window, renderer);
        handle_mutation_input(&window, &mut mesh, &mut mouse);
        handle_camera_input(&window, &mut camera);

        window.update_with_buffer(buffer.get_pixels(), buffer.width, buffer.height).unwrap();
        print!("\x1b[7Hframe time: {ftime: >3} ms", ftime = framestart.elapsed().as_millis());
    }
    print!("\x1b[0m");
}

