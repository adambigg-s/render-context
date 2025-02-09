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

    pub fn get_texture(&self, x: Float, y: Float) -> Color {
        let idx = self.idx(x, y);
        self.texture[idx]
    }

    #[inline]
    fn get_width(&self) -> Float {
        self.width as Float
    }

    #[inline]
    fn get_height(&self) -> Float {
        self.height as Float
    }

    #[inline]
    fn idx(&self, x: Float, y: Float) -> usize {
        let nx = ((x * self.get_width()) as usize)
            .clamp(0, self.width-1);
        let ny = ((y * self.get_height()) as usize)
            .clamp(0, self.height-1);
        // let nx = ((x * self.get_width()) as usize).min(self.width-1);
        // let ny = ((y * self.get_height()) as usize).min(self.height-1);

        {
            debug_assert!(self.inbounds(nx, ny), "index: {},{} dims: {},{}", nx, ny, self.width, self.height);
        }

        ny * self.width + nx
    }

    #[inline]
    fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
