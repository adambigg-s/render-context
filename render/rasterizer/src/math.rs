#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



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
    fn floatify(self) -> Float { self }
}

impl Floatify for u8 {
    fn floatify(self) -> Float {
        self as Float
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Vec2i {
    pub x: Int, pub y: Int,
}

impl Vec2i {
    pub fn cons(x: Int, y: Int) -> Vec2i {
        Vec2i { x, y }
    }

    pub fn fromvec2u(vec: Vec2u) -> Vec2i {
        Vec2i::cons(vec.x as Int, vec.y as Int)
    }

    pub fn det(&self, other: &Self) -> Int {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Vec2i {
    type Output = Vec2i;
    fn add(self, other: Vec2i) -> Self::Output {
        Vec2i::cons(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;
    fn sub(self, other: Vec2i) -> Self::Output {
        Vec2i::cons(self.x - other.x, self.y - other.y)
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Vec2u {
    pub x: usize, pub y: usize,
}

impl Vec2u {
    pub fn cons(x: usize, y: usize) -> Vec2u {
        Vec2u { x, y }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Vec3i {
    pub x: Int, pub y: Int, pub z: Int,
}

impl Vec3i {
    pub fn cons(x: Int, y: Int, z: Int) -> Vec3i {
        Vec3i { x, y, z }
    }

    pub fn detxy(&self, other: &Self) -> Int {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Vec3i {
    type Output = Vec3i;
    fn add(self, other: Self) -> Self::Output {
        Vec3i::cons(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3i {
    type Output = Vec3i;
    fn sub(self, other: Self) -> Self::Output {
        Vec3i::cons(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Vec3f {
    pub x: Float, pub y: Float, pub z: Float,
}

impl Vec3f {
    pub fn cons<T>(x: T, y: T, z: T) -> Vec3f
    where T: Floatify {
        Vec3f { x: x.floatify(), y: y.floatify(), z: z.floatify() }
    }

    pub fn rotatex(&mut self, a: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = a.sin_cos();
        self.x = x;
        self.y = y * cos - z * sin;
        self.z = y * sin + z * cos;
    }

    pub fn rotatey(&mut self, b: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = b.sin_cos();
        self.x = x * cos + z * sin;
        self.y = y;
        self.z = -x * sin + z * cos;
    }

    pub fn rotatez(&mut self, c: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = c.sin_cos();
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
        self.z = z;
    }

    pub fn rotationmatxyz(&mut self, angles: Vec3f) {
        self.rotatex(angles.x);
        self.rotatey(angles.y);
        self.rotatez(angles.z);
    }

    pub fn rotationmatzyx(&mut self, angles: Vec3f) {
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

    pub fn inner_prod(&self, other: &Vec3f) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&mut self) {
        let length = self.inner_prod(self).sqrt();
        self.x /= length; self.y /= length; self.z /= length;
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3f::cons(
            self.y * other.z - self.z * other.y,
            self.x * other.z - self.z * other.x,
            self.x * other.y - self.y * other.x
        )
    }
}



impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, other: Vec3f) -> Self::Output {
        Vec3f::cons(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3f {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x; self.y -= other.y; self.z -= other.z;
    }
}



impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, other: Vec3f) -> Self::Output {
        Vec3f::cons(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x; self.y += other.y; self.z += other.z;
    }
}



impl Mul<Float> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: Float) -> Self::Output {
        Vec3f::cons(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<Float> for Vec3f {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs; self.y *= rhs; self.z *= rhs;
    }
}



impl Div<Float> for Vec3f {
    type Output = Vec3f;
    fn div(self, rhs: Float) -> Self::Output {
        Vec3f::cons(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<Float> for Vec3f {
    fn div_assign(&mut self, other: Float) {
        self.x /= other; self.y /= other; self.z /= other;
    }
}
