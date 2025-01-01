


pub type Color = u32;

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
        Buffer { height, width, pixels: vec![0xFF000000; height * width] }
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.height
    }

    pub fn pixels(&self) -> &Vec<Color>
    {
        &self.pixels
    }

    pub fn blackout(&mut self)
    {
        self.pixels.iter_mut().for_each(|pix| *pix = 0xFF000000);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, data: Color)
    {
        let (width, height): (usize, usize) = (self.width, self.height);
        let ytransformed: usize = self.height-1 - y;
        {
            debug_assert!(ytransformed * width + x < width * height);
        }
        self.pixels[ytransformed * width + x] = data;
    }
}
