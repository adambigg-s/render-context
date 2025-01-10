


use crate::renderer::TextureData;
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

    pub fn goto(&mut self, target: &str, system: &System) {
        system.planets.iter().for_each(|planet| {
            if planet.name == target {
                self.pos = planet.loc + Vec3::cons(-100.0 - planet.rad, 0.0, 0.0);
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

pub struct PlanetParams {
    pub tilt: Float,
    pub rotation: Float,
}

impl PlanetParams {
    pub fn cons(tilt: Float, rotation: Float) -> PlanetParams {
        PlanetParams { tilt, rotation }
    }
}

pub struct Planet {
    pub name: String,
    pub loc: Vec3,
    pub rad: Float,
    pub texture: Option<TextureData>,
    pub lightsource: bool,
    pub params: Option<PlanetParams>,
    pub features: Vec<Feature>,
}

impl Planet {
    pub fn cons(
        name: String, loc: Vec3, rad: Float, texpath: Option<&str>,
        lightsource: bool, params: Option<PlanetParams>
    ) -> Planet {
        let texture = texpath.map(TextureData::from);
        Planet { name, loc, rad, texture, lightsource, params, features: Vec::new() }
    }
}

pub enum Feature {
    Orbit(Orbit),
    Ring(Ring),
    SpacialReference(SpacialReference),
    Moon(Planet),
}

pub struct Orbit {
    pub semimajor: Float,
    pub eccentricity: Float,
    pub inclination: Float,
    pub longofascnode: Float,
    pub argofperi: Float,
}

impl Orbit {
    pub fn cons(semimajor: Float, eccentricity: Float, inclination: Float,
        longofascendingnode: Float, argofperi: Float
    ) -> Orbit {
        Orbit {
            semimajor, eccentricity, inclination, longofascnode: longofascendingnode, argofperi
        }
    }
}

pub struct Ring {
    pub rad: Float,
    pub depth: Float,
    pub texture: TextureData,
    pub params: Option<PlanetParams>,
}

impl Ring {
    pub fn cons(rad: Float, depth: Float, texpath: &str, params: Option<PlanetParams>) -> Ring {
        let texture = TextureData::from(texpath);
        Ring { rad, depth, texture, params }
    }
}

pub struct SpacialReference {
    pub length: Float,
}

impl SpacialReference {
    pub fn cons(length: Float) -> SpacialReference {
        SpacialReference { length }
    }
}

pub struct System {
    pub planets: Vec<Planet>,
    pub lightsources: Vec<Vec3>,
}

#[allow(dead_code)]
impl System {
    pub fn from(planet: Planet) -> System {
        let source = planet.loc;
        System { planets: vec![planet], lightsources: vec![source] }
    }

    pub fn add_planet(&mut self, planet: Planet) {
        if planet.lightsource {
            self.lightsources.push(planet.loc);
        }
        self.planets.push(planet);
    }

    pub fn add_feature(&mut self, target: &str, feature: Feature) {
        if let Some(planet) = self.planets.iter_mut().find(|planet| planet.name == target) {
            planet.features.push(feature);
        }
    }

    pub fn add_spaceref(&mut self, target: &str, spaceref: SpacialReference) {
        if let Some(planet) = self.planets.iter_mut().find(|planet| planet.name == target) {
            planet.features.push(Feature::SpacialReference(spaceref));
        }
    }

    pub fn add_orbit(&mut self, target: &str, orbit: Orbit) {
        if let Some(planet) = self.planets.iter_mut().find(|planet| planet.name == target) {
            planet.features.push(Feature::Orbit(orbit));
        }
    }

    pub fn add_ring(&mut self, target: &str, ring: Ring) {
        if let Some(planet) = self.planets.iter_mut().find(|planet| planet.name == target) {
            planet.features.push(Feature::Ring(ring));
        }
    }
}
