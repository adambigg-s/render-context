


mod math;
mod renderer;
mod geometry;



use minifb::{Key, Scale, Window, WindowOptions};

use geometry::{Mesh, Triangle};
use math::Vec3;
use renderer::{Buffer, Renderer};



type Float = f32;
type Int = i32;
type Color = u32;



const SCREENSCALE: Float = 200.0;
const TERMHEIGHTWIDTH: Float = 1.0;



fn main() {
    let mut buffer = Buffer::cons(500, 900);
    let camera = Vec3::cons(-150, 0, 0);

    let tri1 = Triangle::cons(Vec3::cons(0, 0, 50), Vec3::cons(0, 0, -50), Vec3::cons(60, 80, 0));
    let tri2 = Triangle::cons(Vec3::cons(0, 0, 50), Vec3::cons(0, 0, -50), Vec3::cons(-60, -80, 0));
    let tri3 = Triangle::cons(Vec3::cons(0, 0, 50), Vec3::cons(0, 0, -50), Vec3::cons(-60, 80, 0));
    let tri4 = Triangle::cons(Vec3::cons(0, 0, 50), Vec3::cons(0, 0, -50), Vec3::cons(60, -80, 0));
    let mut mesh = Mesh::cons(vec![tri1], Vec3::cons(0, 0, 0));
    mesh.tris.push(tri2);
    mesh.tris.push(tri3);
    mesh.tris.push(tri4);
    
    let mut mesh = Mesh::build_from_file("icosahedron.vert", 70.0);
    
    print!("\x1b[?25l");
    
    let mut window = Window::new(
        "rasterization methods testing",
        buffer.width,
        buffer.height,
        WindowOptions { scale: Scale::X2, ..Default::default() }
    ).unwrap();
    window.set_target_fps(100);

    while !window.is_key_down(Key::Escape) && !window.is_key_down(Key::C) {
        buffer.clear();

        {
            if !window.is_key_down(Key::R) {
                mesh.rotatex(0.005);
                mesh.rotatey(0.004);
                mesh.rotatez(0.008);
            }
        }

        let mut renderer = Renderer::cons(&mut buffer, &mesh, &camera);
        renderer.draw_bounding_box(0xffffaacc);
        renderer.render_mesh();

        window.update_with_buffer(&buffer.pixels, buffer.width, buffer.height).unwrap();
    }
}
