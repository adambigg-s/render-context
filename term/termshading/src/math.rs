#![allow(dead_code)]



use std::ops::{Add, Sub};

use crate::Float;



#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: Float, pub y: Float, pub z: Float,
}

impl Vec3 {
    pub fn cons<T>(x: T, y: T, z: T) -> Vec3
    where T: Into<Float> {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }

    pub fn rotatex(&mut self, a: Float) {
        let x = self.x; let y = self.y; let z = self.z;
        let (sin, cos) = a.sin_cos();
        self.x = x;
        self.y = y * cos - z * sin;
        self.z = y * sin + z * cos;
    }

    pub fn rotatey(&mut self, b: Float) {
        let x = self.x; let y = self.y; let z = self.z;
        let (sin, cos) = b.sin_cos();
        self.x = x * cos + z * sin;
        self.y = y;
        self.z = -x * sin + z * cos;
    }

    pub fn rotatez(&mut self, c: Float) {
        let x = self.x; let y = self.y; let z = self.z;
        let (sin, cos) = c.sin_cos();
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
        self.z = z;
    }

    pub fn rotationmatxyz(&mut self, angles: Vec3) {
        self.rotatex(angles.x);
        self.rotatey(angles.y);
        self.rotatez(angles.z);
    }

    pub fn dot(&self, other: &Vec3) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self) {
        let length = self.dot(self).sqrt();
        self.x /= length; self.y /= length; self.z /= length;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::cons(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        Vec3::cons(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}



#[cfg(test)]
mod test {
    use crate::Int;
    use super::*;

    #[test]
    fn norm() {
        let mut v1 = Vec3::cons(1.0, 0.0, 0.0);
        v1.normalize();
        assert!(v1.x as Int == 1);
        assert!(v1.y as Int == 0);
        assert!(v1.z as Int == 0);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::cons(1.0, 0.0, 0.0);
        let v2 = Vec3::cons(1.0, 0.0, 0.0);
        let test = v1.dot(&v2);
        assert!(test as Int == 1);
    }
}
