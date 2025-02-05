


mod math;
mod renderer;
mod geometry;
mod render_utils;



use std::time::Instant;

use minifb::{Key, Scale, Window, WindowOptions};

use geometry::{Mesh, RefFrame, Trif};
use math::Vec3f;
use render_utils::{Buffer, Camera, Color};
use renderer::Renderer;



type Float = f32;
type Int = i32;



const FOV: Float = 90.;
const BACKGROUND: u32 = 0xffdddddd;



#[allow(unused_variables, unused_mut)]
fn main() {
    let mut buffer = Buffer::cons(1440 / 3, 2560 / 3);
    let frame = RefFrame::cons(Vec3f::cons(-20, -80, 20), 10.);
    let camera = Camera::cons(Vec3f::cons(-200, 0, 0));

    let tri = Trif::cons(Vec3f::cons(0, -50, 20), Vec3f::cons(0, 50, 20), Vec3f::cons(0, 0, -50));
    let tri2 = Trif::cons(Vec3f::cons(0, 50, 20), Vec3f::cons(0, 50, -20), Vec3f::cons(0, 0, -50));
    let mut mesh = Mesh::cons(vec![tri], Vec3f::cons(0, 0, 0));
    mesh.tris.push(tri2);
    let mut mesh = Mesh::build_from_file("icosahedron.vert", 55.);
    let mut mesh = Mesh::build_from_file_extended("portal.obj", 100.);
    
    let mut window = Window::new(
        "rasterization methods testing",
        buffer.width,
        buffer.height,
        WindowOptions { scale: Scale::X2, ..Default::default() }
    ).unwrap();
    window.set_target_fps(120);

    while !window.is_key_down(Key::Escape) && !window.is_key_down(Key::C) {
        let framestart = Instant::now();
        buffer.clear();

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
            mesh.rotatex(0.04);
            mesh.rotatey(0.03);
            mesh.rotatez(0.05)
        }

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera, FOV);
        renderer.render_mesh();

        window.update_with_buffer(buffer.get_pixels(), buffer.width, buffer.height).unwrap();
        
        print!("\x1b[7Hframe time: {ftime: >3} ms", ftime = framestart.elapsed().as_millis());
    }
    print!("\x1b[0m");
}
