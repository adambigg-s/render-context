use minifb::Key;

use crate::{math::{pi, Vec3}, viewmodel::ViewModel, Float};

pub struct Wall {
    pub edge1: Vec3<Float>,
    pub edge2: Vec3<Float>,
    pub height: Float,
}

impl Wall {
    pub fn cons(edge1: Vec3<Float>, edge2: Vec3<Float>, height: Float) -> Wall {
        Wall {
            edge1,
            edge2,
            height,
        }
    }
}

pub fn idx_usize_isize(x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize) {
    ((x as isize + dx) as usize, (y as isize + dy) as usize)
}

#[allow(dead_code)]
pub fn delay(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time));
}

#[inline]
pub fn dump<Any>(_thing: Any) {}

pub fn get_movement(view: &mut ViewModel, keys: Vec<Key>) {
    keys.iter().for_each(|key| {
        match key {
            Key::Q => view.rotate(1.0 / pi() / 3.0),
            Key::E => view.rotate(-1.0 / pi() / 3.0),
            Key::W => view.move_forward(1.0, 5.0),
            Key::S => view.move_forward(-1.0, 5.0),
            Key::A => view.move_lateral(1.0, 5.0),
            Key::D => view.move_lateral(-1.0, 5.0),
            Key::R => view.tilt(-1),
            Key::F => view.tilt(1),
            _ => {}
        };
    });
}
