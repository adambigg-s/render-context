


mod math;
mod renderer;
mod geometry;
mod render_utils;
mod utils;
mod texture;



use std::time::Instant;

use minifb::{Key, Scale};

use geometry::{Mesh, RefFrame, Tri};
use math::Vec3f;
use render_utils::{Buffer, Camera, Color};
use renderer::Renderer;
use utils::make_window;



type Float = f32;
type Int = i32;



const FOV: Float = 90.;
const FPS: usize = 120;
const BACKGROUND: u32 = 0xffbbbbbb;
const RESMOD: usize = 2;
const HEIGHT: usize = 1080 / RESMOD;
const WIDTH: usize  = 1920 / RESMOD;



#[allow(unused_variables, unused_mut)]
fn main() {
    {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    let mut buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut window = make_window(&buffer, FPS, Scale::X2);
    let frame = RefFrame::cons(Vec3f::cons(-20, -80, 20), 10.);
    let camera = Camera::cons(Vec3f::cons(-100, 0, 0));

    let tri1 = Tri::cons(Vec3f::cons(0, -50, 20), Vec3f::cons(0, 50, 20), Vec3f::cons(0, 0, -50));
    let tri2 = Tri::cons(Vec3f::cons(0, 50, 20), Vec3f::cons(0, 50, -20), Vec3f::cons(0, 0, -50));
    let mut mesh = Mesh::cons(vec![tri1], Vec3f::cons(0, -50, 0));
    mesh.tris.push(tri2);
    // let mut mesh = Mesh::build_from_file("icosahedron.vert", 55.);
    // let mut mesh = Mesh::build_from_file_extended("tree.obj", 3.);
    // let mut mesh = Mesh::build_from_file_extended("portal.obj", 55.);
    // let mut mesh = Mesh::build_from_file_extended("plant/plant.obj", 1.);
    // let mut mesh = Mesh::build_from_file_extended("penguin/penguin.obj", 150.);
    mesh.center = Vec3f::cons(0, 0, 0);

    while !window.is_key_down(Key::Escape) && !window.is_key_down(Key::C) {
        let framestart = Instant::now();
        buffer.clear();

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera, FOV);
        renderer.render_mesh();
        
        if window.is_key_down(Key::O) {
            renderer.render_wireframe();
        }
        if !window.is_key_down(Key::R) {
            if window.is_key_down(Key::W) {
                mesh.rotatex(0.02);
            }
            if window.is_key_down(Key::A) {
                mesh.rotatey(0.02);
            }
            if window.is_key_down(Key::Q) {
                mesh.rotatez(0.02);
            }
            if window.is_key_down(Key::S) {
                mesh.rotatex(-0.02);
            }
            if window.is_key_down(Key::D) {
                mesh.rotatey(-0.02);
            }
            if window.is_key_down(Key::E) {
                mesh.rotatez(-0.02);
            }
            // mesh.rotatex(0.03);
            // mesh.rotatey(0.01);
            // mesh.rotatez(0.03);
        }

        window.update_with_buffer(buffer.get_pixels(), buffer.width, buffer.height).unwrap();
        print!("\x1b[7Hframe time: {ftime: >3} ms", ftime = framestart.elapsed().as_millis());
    }
    print!("\x1b[0m");
}
