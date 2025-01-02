


use std::{f32::consts::{PI, TAU}, ops::{AddAssign, Sub}};

use crate::Float;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec3<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
{
    pub fn cons(x: T, y: T, z: T) -> Vec3<T>
    {
        Vec3 { x, y, z }
    }
}

impl<T> AddAssign for Vec3<T>
where T: AddAssign
{
    fn add_assign(&mut self, other: Self)
    {
        self.x += other.x; self.y += other.y; self.z += other.z;
    }
}

impl<T> Sub for Vec3<T>
where T: Sub<Output = T>
{
    type Output = Vec3<T>;
    
    fn sub(self, other: Self) -> Self::Output
    {
        Vec3::cons(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

pub const fn tau() -> Float
{
    TAU
}

pub const fn pi() -> Float
{
    PI
}



#[cfg(test)]
mod test
{
    use super::*;
    
    #[test]
    fn vector_addassign()
    {
        let mut v1 = Vec3::cons(1, 1, 1);
        let v2 = Vec3::cons(12, 12, 12);
        v1 += v2;

        assert!(v1 == Vec3::cons(13, 13, 13));
    }

    #[test]
    fn vector_sub()
    {
        let v1 = Vec3::cons(1, 1, 1);
        let v2 = Vec3::cons(1, 1, 1);

        assert!(v1 - v2 == Vec3::cons(0, 0, 0));
    }
}
