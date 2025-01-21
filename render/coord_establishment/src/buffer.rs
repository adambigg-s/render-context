


use crate::{utility::idx_usize_isize, Color};



pub struct Buffer {
    height: usize,
    width: usize,
    pixels: Vec<Color>,
}

impl Buffer {
    pub fn cons(height: usize, width: usize) -> Buffer {
        Buffer {
            height,
            width,
            pixels: vec![color_tag(99); height * width],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn halfwidth(&self) -> usize {
        self.width / 2
    }

    pub fn halfheight(&self) -> usize {
        self.height / 2
    }

    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn blackout(&mut self) {
        self.pixels.iter_mut().for_each(|pix| *pix = color_tag(99));
    }

    pub fn place_pixel(&mut self, x: usize, y: usize, data: Color) {
        let ytransformed: usize = self.height-1 - y;
        {
            debug_assert!(ytransformed * self.width + x < self.width * self.height);
        }
        self.pixels[ytransformed * self.width + x] = data;
    }

    pub fn draw_cluster(&mut self, x: usize, y: usize, size: isize, color: Color) {
        let range: isize = size / 2;
        (-range..=range).for_each(|dy| {
            (-range..=range).for_each(|dx| {
                let (sx, sy): (usize, usize) = idx_usize_isize(x, y, dx, dy);
                if !self.inbounds(sx, sy) { return; }
                self.place_pixel(sx, sy, color);
            })
        })
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = -(y2 as isize - y1 as isize).abs();
        let start_x = if x1 < x2 { 1 } else { -1 };
        let start_y = if y1 < y2 { 1 } else { -1 };
        let mut error = dx + dy;
        let mut x = x1 as isize;
        let mut y = y1 as isize;
        loop {
            if !self.inbounds(x as usize, y as usize) {
                break;
            }
            self.place_pixel(x as usize, y as usize, color_tag(1));
            if x == x2 as isize && y == y2 as isize {
                break;
            }
            let doubleerror = 2 * error;
            if doubleerror >= dy {
                error += dy;
                x += start_x;
            }
            if doubleerror <= dx {
                error += dx;
                y += start_y;
            }
        }
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

pub fn color_tag(tag: u8) -> Color {
    match tag {
        1 => 0xff00ffff, // cyan
        2 => 0xffff2222, // rusty red
        3 => 0xff666666,
        4 => 0xffff4444, // brighter red
        _ => 0xff000000, // black
    }
}
