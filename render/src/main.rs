


mod buffer;
mod viewmodel;
mod math;



use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

use std::io::{stdout, Write};

use math::Vec3;
use buffer::Buffer;
use viewmodel::ViewModel;



const HEIGHT: usize = 300;
const WIDTH: usize = 400;
const FPS: usize = 120;

type Float = f32;
type Int = i32;

fn main() {
    let mut window: Window = Window::new(
        "3d render context testing <esc> exits",
        WIDTH,
        HEIGHT,
        WindowOptions { scale: Scale::X2, scale_mode: ScaleMode::Stretch, ..Default::default() }
    ).unwrap();
    window.set_target_fps(FPS);
    let mut buffer: Buffer = Buffer::cons(HEIGHT, WIDTH);

    let mut viewmodel: ViewModel = ViewModel::cons(Vec3::cons(0, 0, 0), 0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.blackout();
        get_movement(&mut viewmodel, window.get_keys());

        print!("\r\x1B[2K");
        print!("x: {}, y: {}, z: {}, rot: {}, tilt: {}",
            viewmodel.position.x,
            viewmodel.position.y,
            viewmodel.position.z,
            viewmodel.rotation,
            viewmodel.tilt
        );
        stdout().flush().unwrap();

        window.update_with_buffer(buffer.pixels(), buffer.width(), buffer.height()).unwrap();
    }
}

fn get_movement(view: &mut ViewModel, keys: Vec<Key>)
{
    keys.iter().for_each(|key| {
        match key {
            Key::Q => view.rotate(1),
            Key::E => view.rotate(-1),
            Key::W => view.translate(Vec3::cons(1, 0, 0)),
            Key::S => view.translate(Vec3::cons(-1, 0, 0)),
            Key::A => view.translate(Vec3::cons(0, -1, 0)),
            Key::D => view.translate(Vec3::cons(0, 1, 0)),
            Key::R => view.tilt(1),
            Key::F => view.tilt(-1),
            _ => {},
        };
    })
}
