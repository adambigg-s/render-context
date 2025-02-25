#![allow(dead_code)]



use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{Float, Int};



pub trait Floatify {
    fn floatify(self) -> Float;
}

impl Floatify for Int {
    fn floatify(self) -> Float {
        self as Float
    }
}

impl Floatify for Float {
    fn floatify(self) -> Float {self}
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: Float, pub y: Float, pub z: Float,
}

impl Vec3 {
    pub fn cons<T>(x: T, y: T, z: T) -> Vec3
    where T: Floatify {
        Vec3 { x: x.floatify(), y: y.floatify(), z: z.floatify() }
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

    pub fn rotationmatzyx(&mut self, angles: Vec3) {
        self.rotatez(angles.z);
        self.rotatey(angles.y);
        self.rotatex(angles.x);
    }

    pub fn reflex(&mut self) {
        self.x = -self.x;
    }

    pub fn refley(&mut self) {
        self.y = -self.y;
    }

    pub fn reflez(&mut self) {
        self.z = -self.z;
    }

    pub fn inner_prod(&self, other: &Vec3) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self) {
        let length = self.inner_prod(self).sqrt();
        self.x /= length; self.y /= length; self.z /= length;
    }
}



impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::cons(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x; self.y -= other.y; self.z -= other.z;
    }
}



impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        Vec3::cons(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x; self.y += other.y; self.z += other.z;
    }
}



impl Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Float) -> Self::Output {
        Vec3::cons(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}



impl Div<Float> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Float) -> Self::Output {
        Vec3::cons(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, other: Float) {
        self.x /= other; self.y /= other; self.z /= other;
    }
}

