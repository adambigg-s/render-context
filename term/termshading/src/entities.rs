


use crate::configparser::Config;
use crate::renderer::TextureData;
use crate::{Float, Int, PI, TAU};
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

    pub fn react(&mut self, inputs: &[char], system: &System, config: &mut Config) {
        inputs.iter().for_each(|input| {
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
                '0' => self.goto_default(system),
                'n' => config.toggle_refs(),
                'm' => config.toggle_orbits(),
                ',' => config.modify_fov(1),
                '.' => config.modify_fov(-1),
                _ => {}
            }
        });
    }

    pub fn goto(&mut self, target: &str, system: &System) {
        system.planets.iter().for_each(|planet| {
            if planet.name == target {
                self.pos = planet.loc + Vec3::cons(-100 - (planet.rad as Int * 2), 0, 0);
                self.tilt = 0.0;
                self.rot = 0.0;
            }
        });
    }

    pub fn goto_default(&mut self, system: &System) {
        system.planets.iter().for_each(|planet| {
            if planet.name == "sun" {
                self.pos = planet.loc + Vec3::cons(0, 0, planet.rad as Int * 3);
                self.tilt = -PI / 2.0;
                self.rot = PI / 2.0;
            }
        });
    }

    fn translate(&mut self, dir: Vec3) {
        let mut transdir = dir * self.transspeed;
        let rotation = Vec3::cons(0.0, -self.tilt, self.rot);
        transdir.rotationmatxyz(rotation);
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
    _Moon(Planet),
}

pub struct Orbit {
    pub semimajor: Float,
    pub eccentricity: Float,
    pub inclination: Float,
    pub longitudeascnode: Float,
    pub argofperiapsis: Float,
    pub trueanomaly: Float,
}

impl Orbit {
    pub fn cons(semimajor: Float, eccentricity: Float, inclination: Float,
        longitudeascnode: Float, argofperiapsis: Float, trueanomaly: Float,
    ) -> Orbit {
        Orbit {
            semimajor, eccentricity, inclination, longitudeascnode, argofperiapsis, trueanomaly
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

impl System {
    pub fn from(planet: Planet) -> System {
        let source = planet.loc;
        System { planets: vec![planet], lightsources: vec![source] }
    }

    pub fn transform_mini(&mut self) {
        self.planets.iter_mut().for_each(|planet| {
            planet.rad /= 500.0;
            planet.loc /= 500000.0;
            if planet.name == "sun" {
                planet.rad /= 20.0;
            }
            planet.features.iter_mut().for_each(|feature| {
                match feature {
                    Feature::Orbit(orbit) => orbit.semimajor /= 500.0,
                    Feature::Ring(ring) => { ring.rad /= 500.0; ring.depth /= 500.0; },
                    Feature::SpacialReference(spaceref) => spaceref.length /= 500.0,
                    Feature::_Moon(moon) => moon.rad /= 500.0,
                }
            })
        });
    }

    pub fn add_planet(&mut self, planet: Planet) {
        if planet.lightsource {
            self.lightsources.push(planet.loc);
        }
        self.planets.push(planet);
    }

    pub fn add_feature(&mut self, target: &str, feature: Feature) {
        match feature {
            Feature::SpacialReference(spaceref) => self.add_spaceref(target, spaceref),
            Feature::Ring(ring) => self.add_ring(target, ring),
            Feature::Orbit(orbit) => self.add_orbit(target, orbit),
            Feature::_Moon(moon) => self.add_moon(target, moon),
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

    pub fn add_moon(&mut self, _target: &str, _moon: Planet) {
        todo!();
    }
}
