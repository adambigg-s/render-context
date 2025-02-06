#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use image::open;



pub struct Texture {
    pub height: usize, width: usize,
    texture: Vec<u8>,
}

impl Texture {
    pub fn build_from_file(path: &str) -> Texture {
        let image = open(path).unwrap().to_rgb8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();

        Texture { height: height as usize, width: width as usize, texture: data }
    }
}
