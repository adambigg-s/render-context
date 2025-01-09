


use crate::renderer::{Color, TextureData};
use crate::{Float, PI, TAU};
use crate::math::Vec3;



pub struct ViewModel {
    pub pos: Vec3,
    pub rot: Float,
    pub tilt: Float,
    pub rotspeed: Float,
    pub transspeed: Float,
}

impl ViewModel {
    pub fn new(pos: Vec3) -> ViewModel {
        ViewModel { pos, rot: 0.0, tilt: 0.0, rotspeed: PI / 75.0, transspeed: 16.0 }
    }

    pub fn react(&mut self, inputs: &[char], system: &System) {
        for input in inputs {
            match input {
                'W' => self.translate(Vec3::cons(0, 0, 1)),
                'S' => self.translate(Vec3::cons(0, 0, -1)),
                'a' => self.translate(Vec3::cons(0, -1, 0)),
                'd' => self.translate(Vec3::cons(0, 1, 0)),
                'w' => self.translate(Vec3::cons(1, 0, 0)),
                's' => self.translate(Vec3::cons(-1, 0, 0)),
                'q' => self.rotate(-1.0),
                'e' => self.rotate(1.0),
                'r' => self.tilt(1.0),
                'f' => self.tilt(-1.0),
                '[' => self.transspeed *= 0.5,
                ']' => self.transspeed *= 2.0,
                '1' => self.goto("mercury", system),
                '2' => self.goto("venus", system),
                '3' => self.goto("earth", system),
                '4' => self.goto("mars", system),
                '5' => self.goto("jupiter", system),
                '6' => self.goto("saturn", system),
                '7' => self.goto("uranus", system),
                '8' => self.goto("neptune", system),
                '9' => self.goto("pluto", system),
                _ => {}
            }
        }
    }

    fn goto(&mut self, target: &str, system: &System) {
        system.planets.iter().for_each(|planet| {
            if planet.name == target {
                self.pos = planet.loc + Vec3::cons(-100. - planet.rad, 0.0, 0.0);
                self.tilt = 0.0;
                self.rot = 0.0;
            }
        })
    }

    fn translate(&mut self, dir: Vec3) {
        let mut transdir = dir * self.transspeed;
        transdir.rotatey(-self.tilt);
        transdir.rotatez(self.rot);
        self.pos += transdir;
    }

    fn rotate(&mut self, dir: Float) {
        self.rot += dir * self.rotspeed;
        if self.rot < 0.0 {
            self.rot += TAU;
        }
        else if self.rot > TAU {
            self.rot -= TAU;
        }
    }

    fn tilt(&mut self, dir: Float) {
        self.tilt += dir * self.rotspeed;
        self.tilt = self.tilt.clamp(-PI / 2.0, PI / 2.0);
    }
}

pub struct ObjectParams {
    pub tilt: Float,
    pub rotation: Float,
}

impl ObjectParams {
    pub fn cons(tilt: Float, rotation: Float) -> ObjectParams {
        ObjectParams { tilt, rotation }
    }
}

pub struct Planet {
    pub loc: Vec3,
    pub rad: Float,
    pub texture: Option<TextureData>,
    pub color: Color,
    pub lightsource: bool,
    pub params: Option<ObjectParams>,
    pub name: String,
    pub orbits: Vec<Orbit>,
}

impl Planet {
    pub fn cons(
        loc: Vec3, rad: Float, texpath: Option<&str>, color: Option<Color>,
        lightsource: bool, params: Option<ObjectParams>, name: String,
    ) -> Planet {
        let texture = texpath.map(TextureData::from);
        let color = if let Some(color) = color { color } else { Color::cons(0, 0, 0) };
        Planet { loc, rad, texture, color, lightsource, params, name, orbits: Vec::new() }
    }
}

pub struct Ring {
    pub loc: Vec3,
    pub rad: Float,
    pub depth: Float,
    pub texture: TextureData,
    pub params: Option<ObjectParams>,
}

impl Ring {
    pub fn cons(
        loc: Vec3, rad: Float, depth: Float, texpath: &str, params: Option<ObjectParams>
    ) -> Ring {
        let texture = TextureData::from(texpath);
        Ring { loc, rad, depth, texture, params }
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

pub struct Orbit {
    pub loc: Vec3,
    pub semimajor: Float,
    pub eccentricity: Float,
    pub _inclination: Float,
    pub _longofascendingnode: Float,
    pub _argofperi: Float,
}

impl Orbit {
    pub fn cons(
        loc: Vec3, semimajor: Float, eccentricity: Float, inclination: Float,
        longofascendingnode: Float, argofperi: Float
    ) -> Orbit {
        Orbit {
            loc, semimajor, eccentricity, _inclination: inclination,
            _longofascendingnode: longofascendingnode, _argofperi: argofperi
        }
    }
}

pub struct System {
    pub planets: Vec<Planet>,
    pub spacerefs: Vec<SpacialReference>,
    pub orbits: Vec<Orbit>,
    pub rings: Vec<Ring>,
    pub lightsources: Vec<Vec3>,
}

impl System {
    pub fn from(sphere: Planet) -> System {
        let source = sphere.loc;
        System {
            planets: vec![sphere], spacerefs: Vec::new(), orbits: Vec::new(),
            rings: Vec::new(), lightsources: vec![source],
        }
    }

    pub fn add_planet(&mut self, planet: Planet) {
        if planet.lightsource {
            self.lightsources.push(planet.loc);
        }
        self.planets.push(planet);
    }

    pub fn add_spaceref(&mut self, spaceref: SpacialReference) {
        self.spacerefs.push(spaceref);
    }

    pub fn add_orbit(&mut self, ellipse: Orbit) {
        self.orbits.push(ellipse);
    }

    pub fn add_ring(&mut self, ring: Ring) {
        self.rings.push(ring);
    }
}
