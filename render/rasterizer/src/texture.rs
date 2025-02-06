#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use image::open;

use crate::{render_utils::Color, Float};



pub struct Texture {
    pub height: usize, pub width: usize,
    texture: Vec<Color>,
}

impl Texture {
    pub fn build_from_file(path: &str) -> Texture {
        let image = open(path).unwrap().to_rgb8();
        let (width, height) = image.dimensions();
        let data = image.as_raw();

        let mut texture = Vec::new();
        for window in data.chunks(3) {
            texture.push(Color::cons(window[0], window[1], window[2]));
        }

        Texture { height: height as usize, width: width as usize, texture }
    }

    pub fn get_at(&self, x: Float, y: Float) -> Color {
        let idx = self.idx(x, y);
        self.texture[idx]
    }

    fn idx(&self, x: Float, y: Float) -> usize {
        let nx = (x * self.width as Float).clamp(0., (self.width-1) as Float) as usize;
        let ny = (y * self.height as Float).clamp(0., (self.height-1) as Float) as usize;
        ny * self.width + nx
    }
}
