#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{Float, Int};



pub trait Floatify {
    fn floatify(self) -> Float;
}

impl Floatify for Float {
    fn floatify(self) -> Float { self }
}

impl Floatify for Int {
    fn floatify(self) -> Float {
        self as Float
    }
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

    pub fn from_vec2u(vec: Vec2u) -> Vec2i {
        Vec2i::cons(vec.x as Int, vec.y as Int)
    }

    pub fn determinant(&self, other: &Self) -> Int {
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

    pub fn determinant_xy(&self, other: &Self) -> Int {
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
pub struct Vec2f {
    pub x: Float, pub y: Float,
}

impl Vec2f {
    pub fn cons<T>(x: T, y: T) -> Vec2f
    where T: Floatify {
        Vec2f { x: x.floatify(), y: y.floatify() }
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

    pub fn rot_x(&mut self, a: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = a.sin_cos();
        self.x = x;
        self.y = y * cos - z * sin;
        self.z = y * sin + z * cos;
    }

    pub fn rot_y(&mut self, b: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = b.sin_cos();
        self.x = x * cos + z * sin;
        self.y = y;
        self.z = -x * sin + z * cos;
    }

    pub fn rot_z(&mut self, c: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = c.sin_cos();
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
        self.z = z;
    }

    pub fn inv_rot_x(&mut self, a: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = a.sin_cos();
        self.x = x;
        self.y = y * cos + z * sin;
        self.z = -y * sin + z * cos;
    }

    pub fn inv_rot_y(&mut self, b: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = b.sin_cos();
        self.x = x * cos - z * sin;
        self.y = y;
        self.z = x * sin + z * cos;
    }

    pub fn inv_rot_z(&mut self, c: Float) {
        let Vec3f { x, y, z } = *self;
        let (sin, cos) = c.sin_cos();
        self.x = x * cos + y * sin;
        self.y = -x * sin + y * cos;
        self.z = z;
    }

    pub fn rot_xyz(&mut self, angles: Vec3f) {
        self.rot_x(angles.x);
        self.rot_y(angles.y);
        self.rot_z(angles.z);
    }

    pub fn rot_zyx(&mut self, angles: Vec3f) {
        self.rot_z(angles.z);
        self.rot_y(angles.y);
        self.rot_x(angles.x);
    }

    pub fn inv_rot_xyz(&mut self, angles: Vec3f) {
        self.inv_rot_x(angles.x);
        self.inv_rot_y(angles.y);
        self.inv_rot_z(angles.z);
    }

    pub fn inv_rot_zyx(&mut self, angles: Vec3f) {
        self.inv_rot_z(angles.z);
        self.inv_rot_y(angles.y);
        self.inv_rot_x(angles.x);
    }

    pub fn refl_x(&mut self) {
        self.x = -self.x;
    }

    pub fn refl_y(&mut self) {
        self.y = -self.y;
    }

    pub fn refl_z(&mut self) {
        self.z = -self.z;
    }

    pub fn inner_prod(&self, other: &Vec3f) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn get_normalized(&self) -> Vec3f {
        let mut vec = *self;
        vec.normalize();
        vec
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



impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Self::Output {
        Vec3f::cons(-self.x, -self.y, -self.z)
    }
}
