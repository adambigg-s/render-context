


use crate::Color;



pub struct Buffer
{
    height: usize,
    width: usize,
    pixels: Vec<Color>,
}

impl Buffer
{
    pub fn cons(height: usize, width: usize) -> Buffer
    {
        Buffer { height, width, pixels: vec![color_tag(99); height * width] }
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.height
    }

    pub fn halfwidth(&self) -> usize
    {
        self.width / 2
    }

    pub fn halfheight(&self) -> usize
    {
        self.height / 2
    }

    pub fn pixels(&self) -> &Vec<Color>
    {
        &self.pixels
    }

    pub fn blackout(&mut self)
    {
        self.pixels.iter_mut().for_each(|pix| *pix = color_tag(99));
    }

    pub fn place_pixel(&mut self, x: usize, y: usize, data: Color)
    {
        let ytransformed: usize = self.height-1 - y;
        {
            debug_assert!(ytransformed * self.width + x < self.width * self.height);
        }
        self.pixels[ytransformed * self.width + x] = data;
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool
    {
        x < self.width && y < self.height
    }
}

pub fn color_tag(tag: u8) -> Color
{
    match tag {
        1 => 0xff00ffff, // cyan
        2 => 0xffff2222, // rusty red
        3 => 0xff666666,
        _ => 0xff000000, // black
    }
}
