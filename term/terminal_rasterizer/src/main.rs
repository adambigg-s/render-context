


mod math;
mod renderer;
mod geometry;



use std::{thread::sleep, time::Duration};



use geometry::Triangle;
use math::Vec3;
use renderer::{Buffer, Color, Renderer};



type Float = f32;
type Int = i32;



const SCREENSCALE: Float = 150.0;
const TERMHEIGHTWIDTH: Float = 2.05;



fn main() {
    let mut buffer = Buffer::cons(70, 200);
    let mut fbuffer = String::with_capacity(buffer.width * buffer.height * 30);
    let camera = Vec3::cons(-25, 0, 0);

    let mut tri = Triangle::cons(Vec3::cons(0, 0, 4), Vec3::cons(0, 4, -3), Vec3::cons(0, -5, -1));

    print!("\x1b[?25l");
    loop {
        buffer.clear();

        buffer.set(0, 0, Color::cons(0, 255, 255), 1.);
        buffer.set(buffer.width-1, buffer.height-1, Color::cons(0, 255, 255), 1.);

        let mut renderer = Renderer::cons(&mut buffer, &mut fbuffer, &tri, &camera);
        renderer.render_triangle();
        renderer.render_to_screen();

        tri.rotatex(0.03);
        tri.rotatey(0.03);
        tri.rotatez(0.01);

        sleep(Duration::from_millis(15));
    }
}
