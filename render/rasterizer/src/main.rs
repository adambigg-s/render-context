


mod math;
mod renderer;
mod geometry;



use std::time::Instant;

use minifb::{Key, Scale, Window, WindowOptions};

use geometry::{Mesh, RefFrame};
use math::Vec3;
use renderer::{Buffer, Camera, Renderer};



type Float = f32;
type Int = i32;
type Color = u32;



const SCREENSCALE: Float = 400.0;
const TERMHEIGHTWIDTH: Float = 1.0;



fn main() {
    let mut buffer = Buffer::cons(1080, 1920);
    let frame = RefFrame::cons(Vec3::cons(-20, -80, 20), 10.);
    let camera = Camera::cons(Vec3::cons(-80, 0, 0));

    let mut mesh = Mesh::build_from_file("icosahedron.vert", 55.);
    
    print!("\x1b[?25l");
    let mut window = Window::new(
        "rasterization methods testing",
        buffer.width,
        buffer.height,
        WindowOptions { scale: Scale::X1, ..Default::default() }
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
        }

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera);
        renderer.draw_bounding_box(0xffffaacc);
        renderer.render_mesh(0xffffffff);
        renderer.render_mesh_frame(0xffbbbbbb);
        renderer.render_frame(&frame);
        renderer.render_screen_frame();

        window.update_with_buffer(&buffer.pixels, buffer.width, buffer.height).unwrap();
        
        print!("\x1b[Hframe time: {ftime: >3} ms", ftime = framestart.elapsed().as_millis());
    }
}
