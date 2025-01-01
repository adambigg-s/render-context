


use crate::Int;
use crate::math::Vec3;



pub struct ViewModel
{
    pub position: Vec3<Int>,
    pub rotation: Int,
    pub tilt: Int,
}

impl ViewModel
{
    pub fn cons(position: Vec3<Int>, rotation: Int, tilt: Int) -> ViewModel
    {
        ViewModel { position, rotation, tilt }
    }

    pub fn translate(&mut self, direction: Vec3<Int>)
    {
        self.position += direction;
    }

    pub fn rotate(&mut self, delta: Int)
    {
        self.rotation += delta;
        if self.rotation < 0 {
            self.rotation += 360
        }
    }

    pub fn tilt(&mut self, delta: Int)
    {
        self.tilt += delta;
        self.tilt = self.tilt.clamp(-70, 70);
    }
}
