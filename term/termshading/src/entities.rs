


use crate::renderer::TextureData;
use crate::{Float, PI, TAU};
use crate::math::Vec3;



pub struct ViewModel {
    pub pos: Vec3,
    pub rot: Float,
    pub tilt: Float,
}

impl ViewModel {
    pub fn new() -> ViewModel {
        ViewModel { pos: Vec3::cons(0.0, 0.0, 0.0), rot: 0.0, tilt: 0.0 }
    }

    pub fn react(&mut self, inputs: &[char]) {
        for input in inputs {
            match input {
                'w' => self.translate(Vec3::cons(1.0, 0.0, 0.0)),
                's' => self.translate(Vec3::cons(-1.0, 0.0, 0.0)),
                'a' => self.translate(Vec3::cons(0.0, -1.0, 0.0)),
                'd' => self.translate(Vec3::cons(0.0, 1.0, 0.0)),
                'r' => self.translate(Vec3::cons(0.0, 0.0, 1.0)),
                'f' => self.translate(Vec3::cons(0.0, 0.0, -1.0)),
                'q' => self.rotate(-1.0),
                'e' => self.rotate(1.0),
                't' => self.tilt(1.0),
                'g' => self.tilt(-1.0),
                _ => {}
            }
        }
    }

    fn translate(&mut self, dir: Vec3) {
        let mut transdir = dir;
        transdir.rotatez(self.rot);
        self.pos = self.pos + transdir;
    }

    fn rotate(&mut self, dir: Float) {
        let speed = PI / 100.0;
        self.rot += dir * speed;
        if self.rot < 0.0 {
            self.rot += TAU;
        }
        else if self.rot > TAU {
            self.rot -= TAU;
        }
    }

    fn tilt(&mut self, dir: Float) {
        let speed = PI / 100.0;
        self.tilt += dir * speed;
        self.tilt = self.tilt.clamp(-PI / 2.0, PI / 2.0);
    }
}

pub struct Sphere {
    pub loc: Vec3,
    pub rad: Float,
    pub texture: Option<TextureData>,
}

impl Sphere {
    pub fn cons(loc: Vec3, rad: Float, texpath: Option<&str>) -> Sphere {
        let texture;
        if let Some(path) = texpath {
            texture = Some(TextureData::from(path));
        } else {
            texture = None;
        }
        Sphere { loc, rad, texture }
    }
}

pub struct System {
    pub spheres: Vec<Sphere>,
}

impl System {
    pub fn from(sphere: Sphere) -> System {
        System { spheres: vec![sphere] }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}
