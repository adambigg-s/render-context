


mod math;
mod renderer;
mod geometry;
mod render_utils;
mod utils;



use std::time::Instant;

use minifb::Key;

use geometry::{Mesh, RefFrame, Tri};
use math::Vec3f;
use render_utils::{Buffer, Camera, Color};
use renderer::Renderer;
use utils::{handle_inputs, make_window};



type Float = f32;
type Int = i32;



const FOV: Float = 90.;
const FPS: usize = 90;
const BACKGROUND: u32 = 0xffdddddd;
const RESMOD: usize = 3;
const HEIGHT: usize = 1440 / RESMOD;
const WIDTH: usize  = 2560 / RESMOD;



#[allow(unused_variables, unused_mut)]
fn main() {
    {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut window = make_window(&buffer, FPS);
    let frame = RefFrame::cons(Vec3f::cons(-20, -80, 20), 10.);
    let camera = Camera::cons(Vec3f::cons(-100, 0, 0));

    let tri1 = Tri::cons(Vec3f::cons(0, -50, 20), Vec3f::cons(0, 50, 20), Vec3f::cons(0, 0, -50));
    let tri2 = Tri::cons(Vec3f::cons(0, 50, 20), Vec3f::cons(0, 50, -20), Vec3f::cons(0, 0, -50));
    let mut mesh = Mesh::cons(vec![tri1], Vec3f::cons(0, 0, 0));
    mesh.tris.push(tri2);
    let mut mesh = Mesh::build_from_file("icosahedron.vert", 55.);
    let mut mesh = Mesh::build_from_file_extended("portal.obj", 50.);

    while !window.is_key_down(Key::Escape) && !window.is_key_down(Key::C) {
        let framestart = Instant::now();
        buffer.clear();

        handle_inputs(&window, &mut mesh);
        if !window.is_key_down(Key::R) {
            mesh.rotatex(0.03);
            mesh.rotatey(0.01);
            mesh.rotatez(0.03);
        }

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera, FOV);
        renderer.render_mesh();

        window.update_with_buffer(buffer.get_pixels(), buffer.width, buffer.height).unwrap();
        print!("\x1b[7Hframe time: {ftime: >3} ms", ftime = framestart.elapsed().as_millis());
    }
    print!("\x1b[0m");
}
