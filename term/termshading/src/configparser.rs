


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

use crate::entities::{Feature, Orbit, OrbitalParams, PlanetParams, Ring, SpacialReference};
use crate::entities::{Planet, System};
use crate::math::{orbital_cartesian_transformation, Vec3};
use crate::utils::flash_error;
use crate::{Float, Int};



pub const SUNPATH: &str = "../planet_textures/sun_map.txt";
pub const MERCURYPATH: &str = "../planet_textures/mercury_map.txt";
pub const VENUSPATH: &str = "../planet_textures/venus_map.txt";
pub const EARTHPATH: &str = "../planet_textures/earth_map.txt";
pub const MOONPATH: &str = "../planet_textures/moon_map.txt";
pub const MARSPATH: &str = "../planet_textures/mars_map.txt";
pub const JUPITERPATH: &str = "../planet_textures/jupiter_map.txt";
pub const SATURNPATH: &str = "../planet_textures/saturn_map.txt";
pub const RINGPATH: &str = "../planet_textures/saturn_ring.txt";
pub const URANUSPATH: &str = "../planet_textures/uranus_map.txt";
pub const NEPTUNEPATH: &str = "../planet_textures/neptune_map.txt";
pub const PLUTOPATH: &str = "../planet_textures/pluto_map.txt";



pub struct TargetFeature<'t> {
    target: &'t str,
    feature: Feature,
}

impl<'t> TargetFeature<'t> {
    fn cons(target: &'t str, feature: Feature) -> TargetFeature<'t> {
        TargetFeature { target, feature }
    }
}

pub fn parse_config(file_path: &str, system: &mut System) -> Result<(), Box<dyn Error>> {
    print!("\x1b[2J");
    print!("\x1b[H");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let error_delay = 3000;

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        else if line.starts_with("planet") {
            match parse_planet(&line) {
                Ok(planet) => system.add_planet(planet),
                Err(err) => flash_error(err, error_delay),
            }
        }
        else if line.starts_with("spaceref") {
            match parse_spaceref(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, error_delay),
            }
        }
        else if line.starts_with("orbit") {
            match parse_orbit(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, error_delay),
            }
        }
        else if line.starts_with("ring") {
            match parse_ring(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, error_delay),
            }
        }
        else if line.starts_with("moon") {
            match parse_moon(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, error_delay),
            }
        }
        
        if line.contains("orbital") || line.contains("planet") {
            match parse_parent_orbit(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, error_delay),
            }
        }
    }

    Ok(())
}

fn parse_planet(data: &str) -> Result<Planet, Box<dyn Error>> {
    let mut name = None;
    let mut loc = None;
    let mut rad = None;
    let mut params = None;
    let mut lightsource = false;

    for token in data.split_whitespace() {
        if token == "planet" {
            continue;
        }
        else if name.is_none() {
            name = Some(token);
        }
        else if rad.is_none() {
            rad = Some(token.parse::<Float>()?);
        }
        else if token.contains('=') {
            let parts: Vec<&str> = token.split('=').collect();
            if parts.len() != 2 {
                return Err("unmatched key".into());
            }
            let (key, value) = (parts[0], parts[1]);
            match key {
                "orbital" => {
                    location_orbital(value, &mut loc)?;
                }
                "cartesian" => {
                    location_cartesian(value, &mut loc)?;
                }
                "polar" => {
                    location_polar(value, &mut loc)?;
                }
                "params" => {
                    parse_params_specific(value, &mut params)?;
                }
                "lightsource" => {
                    lightsource = value.parse::<bool>()?;
                }
                _ => {}
            }
        }
    }

    if let (Some(name), Some(loc), Some(rad)) = (name, loc, rad) {
        let texture = get_texture(name);
        Ok(Planet::cons(name.to_owned(), loc, rad, texture, lightsource, params))
    }
    else {
        Err("missing requirements".into())
    }
}

fn parse_spaceref(data: &str) -> Result<TargetFeature, Box<dyn Error>> {
    let mut length = None;
    let mut target = None;
    for token in data.split_whitespace() {
        if token == "spaceref" {
            continue;
        }
        else if target.is_none() {
            target = Some(token);
        }
        else if length.is_none() {
            length = Some(token.parse::<Float>()?);
        }
    }

    if let (Some(length), Some(target)) = (length, target) {
        Ok(TargetFeature::cons(
            target,
            Feature::SpacialReference(SpacialReference::cons(length)),
        ))
    }
    else {
        Err("missing requirements".into())
    }
}

fn parse_orbit(data: &str) -> Result<TargetFeature, Box<dyn Error>> {
    let mut target: Option<&str> = None;
    let mut orbit: Option<Orbit> = None;

    for token in data.split_whitespace() {
        if token == "orbit" {
            continue;
        }
        else if target.is_none() {
            target = Some(token);
        }
        else if let Some(value) = token.strip_prefix("params=") {
            parse_orbit_specific(value, &mut orbit)?;
        }
    }

    if let (Some(orbit), Some(target)) = (orbit, target) {
        Ok(TargetFeature::cons(target, Feature::Orbit(orbit)))
    }
    else {
        Err("error parsing orbit".into())
    }
}

fn parse_ring(data: &str) -> Result<TargetFeature, Box<dyn Error>> {
    let mut target = None;
    let mut rad = None;
    let mut depth = None;
    let mut params = None;

    for token in data.split_whitespace() {
        if token == "ring" {
            continue;
        }
        else if target.is_none() {
            target = Some(token);
        }
        else if token.contains('=') {
            let parts: Vec<&str> = token.split('=').collect();
            if parts.len() != 2 {
                return Err("unmatched key".into());
            }
            let (key, value) = (parts[0], parts[1]);
            match key {
                "dimens" => {
                    parse_ring_specific(value, &mut rad, &mut depth)?;
                }
                "params" => {
                    parse_params_specific(value, &mut params)?;
                }
                _ => {}
            }
        }
    }
    if let (Some(target), Some(rad), Some(depth)) = (target, rad, depth) {
        Ok(TargetFeature::cons(target, Feature::Ring(Ring::cons(rad, depth, RINGPATH, params))))
    }
    else {
        Err("error parsing ring".into())
    }
}

fn parse_moon(data: &str) -> Result<TargetFeature, Box<dyn Error>> {
    let mut name = None;
    let mut target = None;
    let mut loc = None;
    let mut rad = None;
    let mut params = None;
    let mut lightsource = false;

    for token in data.split_whitespace() {
        if token == "moon" {
            continue;
        }
        else if name.is_none() {
            name = Some(token);
        }
        else if rad.is_none() {
            rad = Some(token.parse::<Float>()?);
        }
        else if token.contains('=') {
            let parts: Vec<&str> = token.split('=').collect();
            if parts.len() != 2 {
                return Err("unmatched key".into());
            }
            let (key, value) = (parts[0], parts[1]);
            match key {
                "orbital" => {
                    location_orbital(value, &mut loc)?;
                }
                "cartesian" => {
                    location_cartesian(value, &mut loc)?;
                }
                "polar" => {
                    location_polar(value, &mut loc)?;
                }
                "params" => {
                    parse_params_specific(value, &mut params)?;
                }
                "lightsource" => {
                    lightsource = value.parse::<bool>()?;
                }
                "target" => {
                    target = Some(value);
                }
                _ => {}
            }
        }
    }

    if let (Some(name), Some(loc), Some(rad), Some(target)) = (name, loc, rad, target) {
        let texture = get_texture(name);
        Ok(TargetFeature::cons(target, Feature::Moon(
            Planet::cons(name.to_owned(), loc, rad, texture, lightsource, params))))
    }
    else {
        Err("missing requirements".into())
    }
}

fn parse_parent_orbit(data: &str) -> Result<TargetFeature, Box<dyn Error>> {
    let mut target = None;
    let mut orbit = None;
    for token in data.split_whitespace() {
        if token == "planet" {
            continue;
        }
        else if target.is_none() {
            target = Some(token);
        }
        else if let Some(value) = token.strip_prefix("orbital=") {
            parse_orbit_specific(value, &mut orbit)?;
        }
    }

    if let (Some(target), Some(mut orbit)) = (target, orbit) {
        orbit.apply_lighting = false;
        Ok(TargetFeature::cons(target, Feature::Orbit(orbit)))
    }
    else {
        Err("error parsing sun orbital path".into())
    }
}

fn parse_orbit_specific(value: &str, orbit: &mut Option<Orbit>) -> Result<(), Box<dyn Error>> {
    let split: Vec<&str> = value.split(',').collect();
    if split.len() < 6 {
        return Err("too few arguments".into());
    }
    let semimajor = split[0].parse::<Float>()?;
    let eccentricity = split[1].parse::<Float>()?;
    let inclination = split[2].parse::<Float>()?.to_radians();
    let longitdueofascnode = split[3].parse::<Float>()?.to_radians();
    let argofperiapsis = split[4].parse::<Float>()?.to_radians();
    let trueanomaly = split[5].parse::<Float>()?.to_radians();
    let bary = Vec3::cons(0, 0, 0);
    *orbit = Some(Orbit::cons(OrbitalParams::cons(semimajor, eccentricity, inclination,
        longitdueofascnode, argofperiapsis, trueanomaly), bary, true));
    Ok(())
}

fn parse_ring_specific(value: &str, rad: &mut Option<f32>, depth: &mut Option<f32>) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = value.split(',').collect();
    *rad = Some(parts[0].parse::<Float>()?);
    *depth = Some(parts[1].parse::<Float>()?);
    Ok(())
}

fn parse_params_specific(value: &str, params: &mut Option<PlanetParams>) -> Result<(), Box<dyn Error>> {
    let split: Vec<&str> = value.split(',').collect();
    if split.len() < 2 {
        return Err("too few arguments".into());
    }
    let tilt = split[0].parse::<Float>()?.to_radians();
    let rotation = split[1].parse::<Float>()?.to_radians();
    *params = Some(PlanetParams::cons(tilt, rotation));
    Ok(())
}

fn location_orbital(value: &str, loc: &mut Option<Vec3>) -> Result<(), Box<dyn Error>> {
    let mut orbit: Option<Orbit> = None;
    parse_orbit_specific(value, &mut orbit)?;
    let cartesian = orbital_cartesian_transformation(&orbit.unwrap());
    *loc = Some(cartesian);
    Ok(())
}

fn location_polar(value: &str, loc: &mut Option<Vec3>) -> Result<(), Box<dyn Error>> {
    let split: Vec<&str> = value.split(',').collect();
    if split.len() < 2 {
        return Err("too few arguments".into());
    }
    let dist = split[0].parse::<Int>()?;
    let theta = split[1].parse::<Float>()?.to_radians();
    let mut vec = Vec3::cons(dist, 0, 0);
    vec.rotatez(-theta);
    *loc = Some(vec);
    Ok(())
}

fn location_cartesian(value: &str, loc: &mut Option<Vec3>) -> Result<(), Box<dyn Error>> {
    let split: Vec<&str> = value.split(',').collect();
    if split.len() < 3 {
        return Err("too few arguments".into());
    }
    let x = split[0].parse::<Int>()?;
    let y = split[1].parse::<Int>()?;
    let z = split[2].parse::<Int>()?;
    *loc = Some(Vec3::cons(x, y, z));
    Ok(())
}

fn get_texture(name: &str) -> Option<&str> {
    match name {
        "mercury" => Some(MERCURYPATH),
        "venus" => Some(VENUSPATH),
        "earth" => Some(EARTHPATH),
        "mars" => Some(MARSPATH),
        "jupiter" => Some(JUPITERPATH),
        "saturn" => Some(SATURNPATH),
        "uranus" => Some(URANUSPATH),
        "neptune" => Some(NEPTUNEPATH),
        "pluto" => Some(PLUTOPATH),
        "sun" => Some(SUNPATH),
        "luna" => Some(MOONPATH),
        _ => None,
    }
}

pub struct Config {
    height: Int, width: Int,
    fov: Float,
    render_refs: bool,
    render_orbits: bool,
    termcharaspect: Float,
    orbital_distance: Float,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            height: 80,
            width: 40,
            fov: 20.0,
            render_refs: false,
            render_orbits: false,
            termcharaspect: 2.0,
            orbital_distance: 400.0,
        }
    }
}

impl Config {
    pub fn height(&self) -> Int {
        self.height
    }

    pub fn width(&self) -> Int {
        self.width
    }

    pub fn fov(&self) -> Float {
        self.fov
    }

    pub fn termcharaspect(&self) -> Float {
        self.termcharaspect
    }

    pub fn render_refs(&self) -> bool {
        self.render_refs
    }

    pub fn orbital_distance(&self) -> Float {
        self.orbital_distance
    }

    pub fn render_orbits(&self) -> bool {
        self.render_orbits
    }

    pub fn toggle_refs(&mut self) {
        self.render_refs = !self.render_refs;
    }

    pub fn toggle_orbits(&mut self) {
        self.render_orbits = !self.render_orbits;
    }

    pub fn modify_fov(&mut self, direc: i32) {
        self.fov += direc as Float;
    }
}

pub fn general_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    print!("\x1b[2J");
    print!("\x1b[H");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut config: Config = Config::default();

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(value) = line.strip_prefix("height=") {
            config.height = value.parse()?;
        }
        else if let Some(value) = line.strip_prefix("width=") {
            config.width = value.parse()?;
        }
        else if let Some(value) = line.strip_prefix("fov=") {
            config.fov = value.parse()?;
        }
        else if let Some(value) = line.strip_prefix("char_aspect=") {
            config.termcharaspect = value.parse()?;
        }
        else if let Some(value) = line.strip_prefix("orbital_distance=") {
            config.orbital_distance = value.parse()?;
        }
    }

    Ok(config)
}
