


use std::f32::consts::PI;

use crate::{Float, Int};
use crate::buffer::Buffer;
use crate::math::Vec3;



const TAU: f32 = PI * 2.0;

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

    pub fn translate(&mut self, direction: Vec3<Int>)
    {
        self.position += direction;
    }

    pub fn rotate(&mut self, delta: Float)
    {
        self.rotation += delta;
        if self.rotation < 0.0 {
            self.rotation += TAU;
        }
        else if self.rotation > TAU {
            self.rotation -= TAU;
        }
    }

    pub fn pos_float(&self) -> Vec3<Float>
    {
        Vec3::cons(self.position.x as Float, self.position.y as Float, self.position.z as Float)
    }

    pub fn tilt(&mut self, delta: Int)
    {
        self.tilt += delta;
        self.tilt = self.tilt.clamp(-70, 70);
    }
}

pub fn draw3d(view: &ViewModel, buffer: &mut Buffer)
{
    let (sine, cosine): (Float, Float) = view.rotation.sin_cos();
    let position: Vec3<Float> = view.pos_float();
    
    // let x1 = 40 - view.position.x;
    // let y1 = 10 - view.position.y;
    // let z1 = 0.;

    // let mut wx1: Float = (x1 as Float) * cosine - (y1 as Float) * sine;
    // let mut wy1: Float = (y1 as Float) * cosine + (x1 as Float) * sine;

    // wx1 = wx1 * 200. / wy1 + (buffer.width() as Float) / 2.;
    // wy1 = z1 * 200. / wy1 + (buffer.height() as Float) / 2.;

    // let drawx = wx1 as usize;
    // let drawy = wy1 as usize;

    // if drawx < buffer.width() && drawy < buffer.height() {
    //     buffer.set_pixel(drawx, drawy, 0xFF00FFFF);
    // }
    //

    let x = 40.0 - position.x;
    let y = 10.0 - position.y;

    let wx = x * cosine - y * sine;
    let wy = y * cosine + x * sine;
}

