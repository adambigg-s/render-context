


use std::{fs::File, io::Write};

use image::{imageops::FilterType, GenericImageView};



const PATH: [&str; 10] = [
    "mercury_map.jpg",
    "venus_map.jpg",
    "earth_map.jpg",
    "mars_map.jpg",
    "jupiter_map.jpg",
    "saturn_ring.jpg",
    "saturn_map.jpg",
    "uranus_map.jpg",
    "neptune_map.jpg",
    "pluto_map.jpg",
    ];

const MODE: Mode = Mode::Color;
const WIDTHPRIME: u32 = 720;

const CHARS: &[char] = &[' ', '.', ':', '-', '~', '=', '+', '*', '#', '%', '@'];



type Float = f32;

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Texture,
    Color,
}

fn main() {
    for path in PATH {
        let mut image = image::open(path).expect("bad path");
        let (width, height) = image.dimensions();

        let desired_width = WIDTHPRIME;
        let aspect_ratio = width as Float / height as Float;
        let widthp = if desired_width < width { desired_width } else { width };
        let heightp = (widthp as Float / aspect_ratio) as u32;

        image = image.resize(widthp, heightp, FilterType::Gaussian);

        let mut output = String::new();

        if MODE == Mode::Texture {
            (0..image.height()).for_each(|y| {
                (0..image.width()).for_each(|x| {
                    let pixel = image.get_pixel(x, y);
                    let average = (pixel[0] as Float + pixel[1] as Float + pixel[2] as Float) / 3.0;
                    let index = (average / 255.0 * (CHARS.len()-1) as Float) as usize;
                    output.push(CHARS[index]);
                });
                output.push('\n');
            });
        }
        else if MODE == Mode::Color {
            (0..image.height()).for_each(|y| {
                (0..image.width()).for_each(|x| {
                    let pixel = image.get_pixel(x, y);
                    let code = to_ansi(pixel[0], pixel[1], pixel[2]);
                    output.push_str(&code);
                    output.push(' ');
                });
                output.push('\n');
            })
        }

        let output_name = path[..(path.len() - 4)].to_owned() + ".txt";

        let mut file = File::create(&output_name).expect("bad file creation");
        file.write_all(output.as_bytes()).expect("bad writing to file");

        println!("{}", output_name);
        println!("success");
    }
}

fn to_ansi(r: u8, g: u8, b: u8) -> String {
    format!("{};{};{}", r, g, b)
}
