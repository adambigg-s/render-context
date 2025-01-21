#![allow(dead_code)]



use crate::{math::Vec3, Float};



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


