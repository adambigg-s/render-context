


use crate::renderer::{Color, TextureData};
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
        let speed = 2.0;
        let mut transdir = dir * speed;
        transdir.rotatey(-self.tilt);
        transdir.rotatez(self.rot);
        self.pos = self.pos + transdir;
    }

    fn rotate(&mut self, dir: Float) {
        let speed = PI / 75.0;
        self.rot += dir * speed;
        if self.rot < 0.0 {
            self.rot += TAU;
        }
        else if self.rot > TAU {
            self.rot -= TAU;
        }
    }

    fn tilt(&mut self, dir: Float) {
        let speed = PI / 75.0;
        self.tilt += dir * speed;
        self.tilt = self.tilt.clamp(-PI / 2.0, PI / 2.0);
    }
}

pub struct Sphere {
    pub loc: Vec3,
    pub rad: Float,
    pub texture: Option<TextureData>,
    pub color: Color,
    pub lightsource: bool
}

impl Sphere {
    pub fn cons(
        loc: Vec3, rad: Float, texpath: Option<&str>, color: Color, lightsource: bool
    ) -> Sphere {
        let texture;
        if let Some(path) = texpath {
            texture = Some(TextureData::from(path));
        } else {
            texture = None;
        }
        Sphere { loc, rad, texture, color, lightsource }
    }
}

pub struct SpacialReference {
    pub loc: Vec3,
    pub length: Float,
}

impl SpacialReference {
    pub fn cons(loc: Vec3, length: Float) -> SpacialReference {
        SpacialReference { loc, length }
    }
}

pub struct Ellipse {
    pub loc: Vec3,
    pub semimajor: Float,
    pub eccentricity: Float,
    pub _inclination: Float,
    pub _longofascendingnode: Float,
    pub _argofperi: Float,
}

impl Ellipse {
    pub fn cons(
        loc: Vec3, semimajor: Float, eccentricity: Float, inclination: Float,
        longofascendingnode: Float, argofperi: Float
    ) -> Ellipse {
        Ellipse {
            loc, semimajor, eccentricity, _inclination: inclination,
            _longofascendingnode: longofascendingnode, _argofperi: argofperi
        }
    }
}

pub struct System {
    pub spheres: Vec<Sphere>,
    pub spacerefs: Vec<SpacialReference>,
    pub ellipses: Vec<Ellipse>,
}

impl System {
    pub fn from(sphere: Sphere) -> System {
        System { spheres: vec![sphere], spacerefs: Vec::new(), ellipses: Vec::new() }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_spaceref(&mut self, spaceref: SpacialReference) {
        self.spacerefs.push(spaceref);
    }

    pub fn add_ellipse(&mut self, ellipse: Ellipse) {
        self.ellipses.push(ellipse);
    }
}
