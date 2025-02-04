


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



const SCREENSCALE: Float = 250.0;
const TERMHEIGHTWIDTH: Float = 1.0;
const BACKGROUND: u32 = 0xffdddddd;



#[allow(unused_variables, unused_mut)]
fn main() {
    let mut buffer = Buffer::cons(1080 / 3, 1920 / 3);
    let frame = RefFrame::cons(Vec3f::cons(-20, -80, 20), 10.);
    let camera = Camera::cons(Vec3f::cons(-130, 0, 0));

    let tri = Trif::cons(Vec3f::cons(0, 0, -50), Vec3f::cons(0, 50, 20), Vec3f::cons(0, -50, 20));
    let mut mesh = Mesh::cons(vec![tri], Vec3f::cons(0, 0, 0));
    let mut mesh = Mesh::build_from_file("icosahedron.vert", 55.);
    let mut mesh = Mesh::build_from_file_extended("portal.obj", 55.);
    
    print!("\x1b[?25l");
    let mut window = Window::new(
        "rasterization methods testing",
        buffer.width,
        buffer.height,
        WindowOptions { scale: Scale::X2, ..Default::default() }
    ).unwrap();
    window.set_target_fps(120);

    print!("\x1b[2J");
    while !window.is_key_down(Key::Escape) && !window.is_key_down(Key::C) {
        let framestart = Instant::now();
        buffer.clear();

        {
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
            mesh.rotatex(0.004);
            mesh.rotatey(0.003);
            mesh.rotatez(0.005)
        }

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera);
        renderer.draw_bounding_box(Color::cons(255, 141, 161));
        renderer.render_mesh(Color::cons(50, 255, 255));
        renderer.render_mesh_frame(Color::cons(200, 200, 200));
        renderer.render_frame(&frame);
        renderer.render_screen_frame();

        window.update_with_buffer(buffer.get_pixels(), buffer.width, buffer.height).unwrap();
        
        print!("\x1b[Hframe time: {ftime: >3} ms", ftime = framestart.elapsed().as_millis());
    }
}
