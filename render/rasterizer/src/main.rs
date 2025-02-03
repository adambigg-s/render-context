


mod math;
mod renderer;
mod geometry;



use std::{thread::sleep, time::Duration};



use geometry::Triangle;
use math::Vec3;
use renderer::{Buffer, Renderer, Color};

use crate::geometry::Mesh;



type Float = f32;
type Int = i32;



const SCREENSCALE: Float = 150.0;
const TERMHEIGHTWIDTH: Float = 2.05;



fn main() {
    let mut buffer = Buffer::cons(55, 180);
    let mut fbuffer = String::with_capacity(buffer.width * buffer.height * 30);
    let camera = Vec3::cons(-35, 0, 0);

    let tri1 = Triangle::cons(Vec3::cons(0, 0, 5), Vec3::cons(0, 0, -5), Vec3::cons(6, 8, 0));
    let tri2 = Triangle::cons(Vec3::cons(0, 0, 5), Vec3::cons(0, 0, -5), Vec3::cons(-6, -8, 0));
    let tri3 = Triangle::cons(Vec3::cons(0, 0, 5), Vec3::cons(0, 0, -5), Vec3::cons(-6, 8, 0));
    let tri4 = Triangle::cons(Vec3::cons(0, 0, 5), Vec3::cons(0, 0, -5), Vec3::cons(6, -8, 0));

    let mut mesh = Mesh::cons(vec![tri1], Vec3::cons(0, 0, 0));
    mesh.tris.push(tri2);
    mesh.tris.push(tri3);
    mesh.tris.push(tri4);

    print!("\x1b[?25l");
    loop {
        buffer.clear();

        {
            mesh.rotatex(0.01);
            mesh.rotatey(0.02);
            mesh.rotatez(0.04);
        }

        let mut renderer = Renderer::cons(&mut buffer, &mut fbuffer, &mesh, &camera);
        renderer.draw_bounding_box(Color::cons(255, 142, 172));
        renderer.render_mesh();
        renderer.render_to_screen();

        sleep(Duration::from_millis(25));
    }
}
