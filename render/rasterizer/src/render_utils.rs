


use crate::{Float, BACKGROUND};
use crate::math::Vec3f;



#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8, pub green: u8, pub blue: u8,
}

impl Color {
    pub fn cons(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn to_u32(self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
}

pub struct Buffer {
    pub height: usize, pub width: usize,
    pixels: Vec<u32>,
    depth: Vec<Float>,
}

impl Buffer {
    pub fn cons(height: usize, width: usize) -> Buffer {
        Buffer {
            height, width,
            pixels: vec![0; width * height], depth: vec![1e+9; width * height]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        {
            debug_assert!(self.inbounds(x, y));
        }
        let idx = self.idx(x, y);
        self.pixels[idx] = color.to_u32();
    }

    pub fn get_pixels(&self) -> &Vec<u32> {
        &self.pixels
    }

    pub fn get_half_height(&self) -> Float {
        (self.height / 2) as Float
    }

    pub fn get_half_width(&self) -> Float {
        (self.width / 2) as Float
    }

    pub fn clear(&mut self) {
        self.pixels.fill(BACKGROUND);
        self.depth.fill(1e+9);
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        (self.height-1 - y) * self.width + x
    }
}

#[allow(dead_code)]
pub struct Camera {
    pub position: Vec3f,
    pub rotation: Vec3f,
}

impl Camera {
    pub fn cons(position: Vec3f) -> Camera {
        Camera { position, rotation: Vec3f::cons(0, 0, 0) }
    }
}
