


use std::ops::{Add, AddAssign, Sub};

use crate::Float;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn cons(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, other: Self) -> Self::Output {
        Vec3::cons(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::cons(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Vec3<Float> {
    pub fn rotation_z(&self, theta: Float) -> Vec3<Float> {
        Vec3::cons(
            theta.cos() * self.x - theta.sin() * self.y,
            theta.sin() * self.x + theta.cos() * self.y,
            self.z,
        )
    }
}



#[cfg(test)]
mod test {
    use super::*;
    
    use crate::Int;

    #[test]
    fn vector_addassign() {
        let mut v1 = Vec3::cons(1, 1, 1);
        let v2 = Vec3::cons(12, 12, 12);
        v1 += v2;

        assert!(v1 == Vec3::cons(13, 13, 13));
    }

    #[test]
    fn vector_sub() {
        let v1 = Vec3::cons(1, 1, 1);
        let v2 = Vec3::cons(1, 1, 1);

        assert!(v1 - v2 == Vec3::cons(0, 0, 0));
    }

    #[test]
    fn rotate_z() {
        let v1 = Vec3::cons(1.0, 0.0, 0.0);
        let v2 = v1.rotation_z(90.0);
        let v3 = Vec3::cons(
            v2.x.round() as Int,
            v2.y.round() as Int,
            v2.z.round() as Int,
        );

        println!("{:?}", v3);
        assert!(v3 == Vec3::cons(0, 1, 0));
    }
}
