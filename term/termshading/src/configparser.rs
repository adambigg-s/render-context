


use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

use crate::entities::{Feature, Orbit, PlanetParams, Ring, SpacialReference};
use crate::entities::{Planet, System};
use crate::math::Vec3;
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

pub fn parse_config(file_path: &str, system: &mut System) -> io::Result<()> {
    print!("\x1b[2J");
    print!("\x1b[H");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        else if line.starts_with("planet") {
            match parse_planet(&line) {
                Ok(planet) => system.add_planet(planet),
                Err(err) => flash_error(err, 2000),
            }
        }
        else if line.starts_with("spaceref") {
            match parse_spaceref(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, 2000),
            }
        }
        else if line.starts_with("orbit") {
            match parse_orbit(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, 2000),
            }
        }
        else if line.starts_with("ring") {
            match parse_ring(&line) {
                Ok(targfeat) => system.add_feature(targfeat.target, targfeat.feature),
                Err(err) => flash_error(err, 2000),
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
                "cartesian" => {
                    let split: Vec<&str> = value.split(',').collect();
                    let x = split[0].parse::<Int>()?;
                    let y = split[1].parse::<Int>()?;
                    let z = split[2].parse::<Int>()?;
                    loc = Some(Vec3::cons(x, y, z));
                }
                "orbital" => {
                    let split: Vec<&str> = value.split(',').collect();
                    let dist = split[0].parse::<Float>()?;
                    let angle = split[1].parse::<Float>()?.to_radians();
                    let mut vector = Vec3::cons(dist, 0.0, 0.0);
                    vector.rotatez(-angle);
                    loc = Some(vector);
                }
                "lightsource" => {
                    lightsource = value.parse::<bool>()?;
                }
                "params" => {
                    let split: Vec<&str> = value.split(',').collect();
                    let tilt = split[0].parse::<Float>()?.to_radians();
                    let rotation = split[1].parse::<Float>()?.to_radians();
                    params = Some(PlanetParams::cons(tilt, rotation));
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
        Err("issue parsing planet".into())
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
        Err("issue parsing spacial reference".into())
    }
}

fn parse_orbit(data: &str) -> Result<TargetFeature, Box<dyn Error>> {
    let mut semimajor = None;
    let mut target = None;
    let mut eccentricity = 0.0;
    let mut inclination = 0.0;
    let mut longofascendingnode = 0.0;
    let mut argofperi = 0.0;

    for token in data.split_whitespace() {
        if token == "orbit" {
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
            if let "params" = key {
                let split: Vec<&str> = value.split(',').collect();
                semimajor = Some(split[0].parse::<Float>()?);
                eccentricity = split[1].parse::<Float>()?;
                inclination = split[2].parse::<Float>()?;
                longofascendingnode = split[3].parse::<Float>()?;
                argofperi = split[4].parse::<Float>()?;
            }
        }
    }

    if let (Some(semimajor), Some(target)) = (semimajor, target) {
        Ok(
            TargetFeature::cons(target,
                Feature::Orbit(
                    Orbit::cons(semimajor,eccentricity,inclination,longofascendingnode,argofperi)
                )
            )
        )
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
                    let parts: Vec<&str> = value.split(',').collect();
                    rad = Some(parts[0].parse::<Float>()?);
                    depth = Some(parts[1].parse::<Float>()?);
                }
                "params" => {
                    let parts: Vec<&str> = value.split(',').collect();
                    let tilt = parts[0].parse::<Float>()?;
                    params = Some(PlanetParams::cons(tilt, 0.0));
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
        "moon" => Some(MOONPATH),
        _ => None,
    }
}
