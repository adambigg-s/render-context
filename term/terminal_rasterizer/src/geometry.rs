


use crate::{math::Vec3, Float};



pub struct Triangle {
    pub a: Vec3, pub b: Vec3, pub c: Vec3,
}

#[allow(dead_code)]
impl Triangle {
    pub fn cons(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        Triangle { a, b, c }
    }

    pub fn rotatex(&mut self, angle: Float) {
        self.a.rotatex(angle);
        self.b.rotatex(angle);
        self.c.rotatex(angle);
    }
    
    pub fn rotatey(&mut self, angle: Float) {
        self.a.rotatey(angle);
        self.b.rotatey(angle);
        self.c.rotatey(angle);
    }

    pub fn rotatez(&mut self, angle: Float) {
        self.a.rotatez(angle);
        self.b.rotatez(angle);
        self.c.rotatez(angle);
    }
}
