


use std::{fs::File, io::Write};

use image::{imageops::FilterType, GenericImageView};



const CHARS: &[char] = &[' ', '.', ':', '-', '~', '=', '+', '*', '#', '%', '@'];

type Float = f32;
type UInt = u32;

fn main() {
    let path = "moon_map.jpg";
    let mut image = image::open(path).expect("bad path");
    let (width, height) = image.dimensions();

    let desired_width = 720;
    let aspect_ratio = width as Float / height as Float;
    let widthp = if desired_width < width { desired_width } else { width };
    let heightp = (widthp as Float / aspect_ratio) as UInt;

    image = image.resize(widthp, heightp, FilterType::Gaussian);

    let mut output = String::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let average = (pixel[0] as Float + pixel[1] as Float + pixel[2] as Float) / 3.0;
            let index = (average / 255.0 * (CHARS.len()-1) as Float) as usize;
            output.push(CHARS[index]);
        }
        output.push('\n');
    }

    let mut file = File::create("moon_map.txt").expect("bad file creation");
    file.write_all(output.as_bytes()).expect("bad writing to file");

    println!("success");
}
