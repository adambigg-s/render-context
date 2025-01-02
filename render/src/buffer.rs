


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
        Buffer { height, width, pixels: vec![black(); height * width] }
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
        self.pixels.iter_mut().for_each(|pix| *pix = black());
    }

    pub fn place_pixel(&mut self, x: usize, y: usize, data: Color)
    {
        let (width, height): (usize, usize) = (self.width, self.height);
        let ytransformed: usize = self.height-1 - y;
        {
            debug_assert!(ytransformed * width + x < width * height);
        }
        self.pixels[ytransformed * width + x] = data;
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool
    {
        x < self.width && y < self.height
    }
}

pub fn black() -> Color
{
    0xFF000000
}

pub fn cyan() -> Color
{
    0xFF00FFFF
}

pub fn red() -> Color
{
    0xFFFF2222
}
