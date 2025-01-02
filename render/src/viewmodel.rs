



use crate::buffer::{cyan, red, Buffer, Color};
use crate::{Float, Int};
use crate::math::{tau, Vec3};



pub struct ViewModel
{
    pub position: Vec3<Int>,
    pub rotation: Float,
    pub tilt: Int,
}

impl ViewModel
{
    pub fn cons(position: Vec3<Int>) -> ViewModel
    {
        ViewModel { position, rotation: Float::default(), tilt: Int::default() }
    }

    pub fn move_forward(&mut self, direction: Float, speed: Float)
    {
        self.position.x += (direction * speed * self.rotation.cos()) as Int;
        self.position.y += (direction * speed * self.rotation.sin()) as Int;
    }

    pub fn move_lateral(&mut self, direction: Float, speed: Float)
    {
        self.position.x -= (direction * speed * self.rotation.sin()) as Int;
        self.position.y += (direction * speed * self.rotation.cos()) as Int;
    }
    
    pub fn rotate(&mut self, delta: Float)
    {
        self.rotation += delta;
        if self.rotation < 0.0 {
            self.rotation += tau();
        }
        else if self.rotation > tau() {
            self.rotation -= tau();
        }
    }

    pub fn position_float(&self) -> Vec3<Float>
    {
        Vec3::cons(self.position.x as Float, self.position.y as Float, self.position.z as Float)
    }

    pub fn tilt(&mut self, delta: Int)
    {
        self.tilt += delta;
        self.tilt = self.tilt.clamp(-70, 70);
    }
}

pub fn draw3d_point(viewmodel: &ViewModel, buffer: &mut Buffer)
{
    let object: Vec3<Float> = Vec3::cons(10.0, 10.0, 0.0);
    let (sin, cos): (Float, Float) = viewmodel.rotation.sin_cos();
    let position: Vec3<Float> = viewmodel.position_float();
    let view: Vec3<Float> = object - position;
}

pub fn draw_point(point: &Vec3<Int>, buffer: &mut Buffer)
{
    let sx: usize = (point.x + buffer.halfwidth() as Int) as usize;
    let sy: usize = (point.y + buffer.halfheight() as Int) as usize;

    draw_cluster(sx, sy, 3, red(), buffer);
}

pub fn draw_view(viewmodel: &ViewModel, buffer: &mut Buffer)
{
    let sx: usize = (viewmodel.position.x + buffer.halfwidth() as Int) as usize;
    let sy: usize = (viewmodel.position.y + buffer.halfheight() as Int) as usize;
    draw_cluster(sx, sy, 3, cyan(), buffer);

    let (sin, cos) = viewmodel.rotation.sin_cos();
    for len in 0..10 {
        let nx = (sx as Float + len as Float * cos) as usize;
        let ny = (sy as Float + len as Float * sin) as usize;
        if buffer.inbounds(nx, ny) {
            buffer.place_pixel(nx, ny, red());
        }
    }
}

pub fn draw_cluster(x: usize, y: usize, size: isize, color: Color, buffer: &mut Buffer)
{
    let range = size / 2;
    (-range..=range).for_each(|dy| {
        (-range..=range).for_each(|dx| {
            let sx = (x as isize + dx) as usize;
            let sy = (y as isize + dy) as usize;
            if buffer.inbounds(sx, sy) {
                buffer.place_pixel(sx, sy, color);
            }
        })
    })
}
