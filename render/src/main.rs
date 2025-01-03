


mod buffer;
mod viewmodel;
mod math;
mod renderer;



use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

use std::io::{stdout, Write};

use math::{pi, Vec3};
use buffer::Buffer;
use viewmodel::{draw3d_point, ViewModel, draw_point, draw_view};



const RES: usize = 2;
const HEIGHT: usize = RES * 120;
const WIDTH: usize = RES * 160;
const FPS: usize = 60;

type Float = f32;
type Int = i32;
type Color = u32;

fn main()
{
    let mut debug_window: Window = Window::new(
        "debug console",
        WIDTH,
        HEIGHT,
        WindowOptions { ..Default::default() }
    ).unwrap();
    let mut window: Window = Window::new(
        "3d render context testing <esc> exits",
        WIDTH,
        HEIGHT,
        WindowOptions { scale: Scale::X2, scale_mode: ScaleMode::Stretch, ..Default::default() }
    ).unwrap();
    debug_window.set_target_fps(FPS);
    window.set_target_fps(FPS);
    let mut debug_buffer: Buffer = Buffer::cons(HEIGHT, WIDTH);
    let mut buffer: Buffer = Buffer::cons(HEIGHT, WIDTH);

    let mut viewmodel: ViewModel = ViewModel::cons(Vec3::cons(-10.0, 0.0, 20.0));

    let mut points: Vec<Vec3<Float>> = Vec::new();
    points.push(Vec3::cons(40.0, 10.0, 0.0));
    points.push(Vec3::cons(40.0, -10.0, 0.0));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.blackout();
        get_movement(&mut viewmodel, window.get_keys());
        for &point in &points {
            draw3d_point(&viewmodel, &mut buffer, &point);
        }

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

            draw_view(&viewmodel, &mut debug_buffer);
            for &point in &points {
                draw_point(&point, &mut debug_buffer);
            }

            print!("\r\x1B[2K");
            print!("x: {}, y: {}, z: {}, rot: {}, tilt: {}, sin: {}, cos: {}",
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
                debug_buffer.pixels(), debug_buffer.width(), debug_buffer.height()
            ).unwrap();
        }
    }
}

fn get_movement(view: &mut ViewModel, keys: Vec<Key>)
{
    keys.iter().for_each(|key| {
        match key {
            Key::Q => view.rotate(1.0 / pi() / 3.0),
            Key::E => view.rotate(-1.0 / pi() / 3.0),
            Key::W => view.move_forward(1.0, 5.0),
            Key::S => view.move_forward(-1.0, 5.0),
            Key::A => view.move_lateral(-1.0, 5.0),
            Key::D => view.move_lateral(1.0, 5.0),
            Key::R => view.tilt(1),
            Key::F => view.tilt(-1),
            _ => {},
        };
    });
}
