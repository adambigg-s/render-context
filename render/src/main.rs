mod buffer;
mod math;
mod renderer;
mod utility;
mod viewmodel;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use utility::{get_movement, Wall};
use viewmodel::ViewModel;

use std::io::{stdout, Write};

use buffer::Buffer;
use math::Vec3;
use renderer::Renderer;

use crate::renderer::{draw_point_2d, draw_view_2d, draw_wall_2d};

const RES: usize = 2;
const HEIGHT: usize = RES * 150;
const WIDTH: usize = RES * 200;
const FPS: usize = 60;

type Float = f32;
type Int = i32;
type Color = u32;

fn main() {
    let mut debug_window: Window = Window::new(
        "debug console",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..Default::default()
        },
    ).unwrap();
    let mut window: Window = Window::new(
        "3d render context testing <esc> exits",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    ).unwrap();
    debug_window.set_target_fps(FPS);
    window.set_target_fps(FPS);

    let mut debug_buffer: Buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut buffer: Buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut viewmodel: ViewModel = ViewModel::cons(Vec3::cons(0.0, 0.0, 10.0));

    let points: Vec<Vec3<Float>> = vec![
        Vec3::cons(70.0, 120.0, 0.0), Vec3::cons(70.0, -120.0, 0.0), Vec3::cons(20.0, 20.0, 0.0),
        Vec3::cons(40.0, 20.0, 3.0),
    ];

    let wall: Wall = Wall::cons(
        points[0],
        points[2],
        45.0,
    );
    let wall2: Wall = Wall::cons(
        points[1],
        points[2],
        45.0,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        get_movement(&mut viewmodel, window.get_keys());
        buffer.blackout();
        let mut renderer = Renderer::cons(&viewmodel, &mut buffer);
        for &point in &points {
            renderer.draw3d_point(&point);
        }
        renderer.draw3d_wall(&wall);
        renderer.draw3d_wall(&wall2);

        window.update_with_buffer(buffer.pixels(), buffer.width(), buffer.height()).unwrap();

        {
            debug_buffer.blackout();
            (0..100).for_each(|y| {
                (0..100).for_each(|x| {
                    let xp = x * 15;
                    let yp = y * 15;
                    let color = (((xp + yp) as u32) << 16) | ((yp as u32) << 8) | (xp as u32);
                    debug_buffer.place_pixel(x, y, color);
                });
            });

            draw_view_2d(&viewmodel, &mut debug_buffer);
            for &point in &points {
                draw_point_2d(&mut debug_buffer, &point);
            }
            draw_wall_2d(&mut debug_buffer, &wall);
            draw_wall_2d(&mut debug_buffer, &wall2);

            print!("\033[2J");
            print!("\r\x1B[2K");
            print!(
                "x: {}, y: {}, z: {}, rot: {}, tilt: {}, sin: {}, cos: {}",
                viewmodel.position.x,
                viewmodel.position.y,
                viewmodel.position.z,
                viewmodel.rotation,
                viewmodel.tilt,
                viewmodel.rotation.sin(),
                viewmodel.rotation.cos(),
            );
            stdout().flush().unwrap();

            debug_window.update_with_buffer(
                debug_buffer.pixels(),
                debug_buffer.width(),
                debug_buffer.height(),
            ).unwrap();
        }
    }
}

