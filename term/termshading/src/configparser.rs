


use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

use crate::{entities::{Orbit, ObjectParams, Ring, SpacialReference}, utils::flash_error, Float, Int};
use crate::math::Vec3;
use crate::entities::{Planet, System};



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
                Ok(spaceref) => system.add_spaceref(spaceref),
                Err(err) => flash_error(err, 2000),
            }
        }
        else if line.starts_with("orbit") {
            match parse_orbit(&line) {
                Ok(orbit) => system.add_orbit(orbit),
                Err(err) => flash_error(err, 2000),
            }
        }
        else if line.starts_with("ring") {
            match parse_ring(&line) {
                Ok(ring) => system.add_ring(ring),
                Err(err) => flash_error(err, 2000),
            }
        }
    }

    Ok(())
}

fn parse_planet(data: &str) -> Result<Planet, Box<dyn Error>> {
    let parts: Vec<&str> = data.split_whitespace().collect();
    if parts.len() < 5 { return Err("not enough inputs: skipping planet".into()); }
    let texture = match parts[1] {
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
        _ => None
    };
    let rad = parts[2].parse::<Float>()?;
    let distance = parts[3].parse::<Int>()?;
    let theta = parts[4].parse::<Float>()?.to_radians();
    let inclination = if let Some(inclination) = parts.get(5) {
        inclination.parse::<Float>()?.to_radians()
    }
    else {
        0.0
    };
    let mut loc = Vec3::cons(distance, 0, 0);
    loc.rotatez(-theta);
    loc.rotatex(-inclination);
    let tilt = if let Some(tilt) = parts.get(6) {
        Some(tilt.parse::<Float>()?.to_radians())
    }
    else {
        None
    };
    let rotation = if let Some(rotation) = parts.get(7) {
        Some(rotation.parse::<Float>()?.to_radians())
    }
    else {
        None
    };
    let params = if let (Some(tilt), Some(rotation)) = (tilt, rotation) {
        Some(ObjectParams::cons(tilt, rotation))
    }
    else {
        None
    };
    let name = parts[1].to_string();
    Ok(Planet::cons(loc, rad, texture, None, false, params, name))
}

fn parse_spaceref(data: &str) -> Result<SpacialReference, Box<dyn Error>> {
    let parts: Vec<&str> = data.split_whitespace().collect();
    if parts.len() < 5 { return Err("not enough inputs: skipping spaceref".into()); }
    let length = parts[1].parse::<Float>()?;
    let (x, y, z) =
        (parts[2].parse::<Float>()?, parts[3].parse::<Float>()?, parts[4].parse::<Float>()?);
    let loc = Vec3::cons(x, y, z);
    Ok(SpacialReference::cons(loc, length))
}

fn parse_ring(data: &str) -> Result<Ring, Box<dyn Error>> {
    let parts: Vec<&str> = data.split_whitespace().collect();
    if parts.len() < 5 { return Err("not enough inputs: skipping ring".into()); }
    let rad = parts[1].parse::<Float>()?;
    let depth = parts[2].parse::<Float>()?;
    let distance = parts[3].parse::<Int>()?;
    let theta = parts[4].parse::<Float>()?.to_radians();
    let inclination = if let Some(inclination) = parts.get(5) {
        inclination.parse::<Float>()?.to_radians()
    }
    else {
        0.0
    };
    let tilt = if let Some(tilt) = parts.get(6) {
        Some(tilt.parse::<Float>()?.to_radians())
    }
    else {
        None
    };
    let rotation = if let Some(rotation) = parts.get(7) {
        Some(rotation.parse::<Float>()?.to_radians())
    }
    else {
        None
    };
    let params = if let (Some(tilt), Some(rotation)) = (tilt, rotation) {
        Some(ObjectParams::cons(tilt, rotation))
    }
    else {
        None
    };
    let mut loc = Vec3::cons(distance, 0, 0);
    loc.rotatez(-theta);
    loc.rotatex(-inclination);
    Ok(Ring::cons(loc, rad, depth, RINGPATH, params))
}

fn parse_orbit(data: &str) -> Result<Orbit, Box<dyn Error>> {
    let parts: Vec<&str> = data.split_whitespace().collect();
    if parts.len() < 8 { return Err("not enough inputs: skipping orbit".into()); }
    let distance = parts[1].parse::<Int>()?;
    let theta = parts[2].parse::<Float>()?.to_radians();
    let mut loc = Vec3::cons(distance, 0, 0);
    loc.rotatez(-theta);
    let semimajor = parts[3].parse::<Float>()?;
    let eccentricity = parts[4].parse::<Float>()?;
    let inclination = parts[5].parse::<Float>()?;
    let longofascendingnode = parts[6].parse::<Float>()?;
    let argofperi = parts[7].parse::<Float>()?;
    Ok(Orbit::cons(loc, semimajor, eccentricity, inclination, longofascendingnode, argofperi))
}
