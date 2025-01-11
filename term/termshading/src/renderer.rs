


use std::fs::read_to_string;
use std::io::{stdout, Write};

use crate::configparser::Config;
use crate::entities::{Feature, Orbit, PlanetParams, Ring, SpacialReference};
use crate::{Float, Int, Planet, System, ViewModel, PI, TAU};
use crate::math::Vec3;



pub struct Renderer<'d> {
    pub viewmodel: &'d ViewModel,
    pub buffer: &'d mut Buffer,
    pub system: &'d System,
    pub config: &'d Config,
}

impl<'d> Renderer<'d> {
    pub fn cons(
        view: &'d ViewModel, buff: &'d mut Buffer, sys: &'d System, config: &'d Config
    ) -> Renderer<'d> {
        Renderer { viewmodel: view, buffer: buff, system: sys, config }
    }

    pub fn render_planets(&mut self) {
        self.system.planets.iter().for_each(|planet| {
            self.render_planet(planet);
        });
    }

    pub fn render_rings(&mut self) {
        self.system.planets.iter().for_each(|planet| {
            planet.features.iter().for_each(|feature| {
                if let Feature::Ring(ring) = feature {
                    self.render_ring(ring, planet);
                }
            });
        });
    }

    pub fn render_orbits(&mut self) {
        self.system.planets.iter().for_each(|planet| {
            planet.features.iter().for_each(|feature| {
                if let Feature::Orbit(orbit) = feature {
                    self.render_orbit(orbit, planet);
                }
            });
        });
    }

    pub fn render_spacerefs(&mut self) {
        self.system.planets.iter().for_each(|planet| {
            planet.features.iter().for_each(|feature| {
                if let Feature::SpacialReference(spaceref) = feature {
                    self.render_spaceref(spaceref, planet);
                }
            });
        });
    }

    fn render_ring(&mut self, ring: &Ring, planet: &Planet) {
        let distance = self.distance_square(&planet.loc).sqrt() - ring.rad;
        if self.behind_view(&planet.loc) || distance / ring.rad > 100.0 { return; }
        let thetadelta = (distance / (ring.rad * 200.0)).max(0.01);
        let gammadelta = (distance / (ring.depth * 10.0)).max(0.3);
        let thetastep = (TAU / thetadelta) as Int;
        let gammastep = (ring.depth / gammadelta) as Int;

        for gammamul in 0..gammastep {
            let gamma = gammamul as Float * gammadelta;
            for thetamul in 0..thetastep {
                let theta = thetamul as Float * thetadelta;

                let rad = ring.rad + gamma;
                let mut worldframe = Vec3::cons(rad * theta.cos(), rad * theta.sin(), 0.0);
                if let Some(params) = &ring.params {
                    self.apply_param_transformations(&mut worldframe, params);
                }
                worldframe += planet.loc;

                let viewframe = self.world_to_view(&worldframe);
                if viewframe.x <= 0.0 { continue; }
                let (screenx, screeny) = self.view_to_screen(&viewframe);

                if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
                    if viewframe.x >= self.buffer.depth[idx] { continue; }
                    
                    let color = self.map_texture_ring(theta, gamma, ring);
                    self.buffer.set(idx, color, viewframe.x, None);
                }
            }
        }
    }

    fn render_orbit(&mut self, orbit: &Orbit, planet: &Planet) {
        let distance = self.distance_square(&planet.loc).sqrt();
        if self.behind_view(&planet.loc) || distance > 1000.0 { return; }
        let thetadelta = (distance / (orbit.semimajor * 170.0)).max(0.01);
        let thetastep = (TAU / thetadelta) as Int;

        for thetamul in 0..thetastep {
            let theta = thetamul as Float * thetadelta;

            let rad = orbit.semimajor
                * (1.0 - orbit.eccentricity * orbit.eccentricity)
                / (1.0 + orbit.eccentricity * theta.cos());

            let x = rad * theta.cos();
            let y = rad * theta.sin();
            let mut worldframe = Vec3::cons(x, y, 0.0);
            let rotations = Vec3::cons(orbit.longitudeascnode, orbit.inclination, orbit.argofperiapsis);
            worldframe.rotationmatzyx(rotations);
            worldframe += planet.loc;

            let viewframe = self.world_to_view(&worldframe);
            if viewframe.x <= 0.0 { continue; }
            let (screenx, screeny) = self.view_to_screen(&viewframe);

            if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
                if viewframe.x > self.buffer.depth[idx] { continue; }
                let mut normal = worldframe - planet.loc;
                normal.normalize();
                let luminance = self.generalize_luminance(worldframe, normal);
                let mut color = Color::cons(204, 174, 6);
                if (theta - orbit.trueanomaly).abs() < 0.05 {
                    color = Color::cons(0, 255, 255);
                }
                else {
                    color.lighting(luminance);
                }
                self.buffer.set(idx, color, viewframe.x, None);
            }
        }
    }

    fn render_spaceref(&mut self, spaceref: &SpacialReference, planet: &Planet) {
        let distance = self.distance_square(&planet.loc).sqrt();
        if self.behind_view(&planet.loc) || distance > spaceref.length * 20.0 { return; }
        let delta = 1.0 / 2.0;
        let deltastep = (spaceref.length / delta) as Int;
        
        for deltamul in 0..deltastep {
            let axisdelta = deltamul as Float * delta;
            self.axis_assistant(
                planet, Vec3::cons(axisdelta, 0.0, 0.0), Color::cons(255, 10, 10));
            self.axis_assistant(
                planet, Vec3::cons(0.0, axisdelta, 0.0), Color::cons(10, 255, 10));
            self.axis_assistant(
                planet, Vec3::cons(0.0, 0.0, axisdelta), Color::cons(10, 10, 255));
        }
    }

    fn axis_assistant(&mut self, planet: &Planet, delta: Vec3, color: Color) {
        let mut worldframe = delta;
        if let Some(params) = &planet.params {
            worldframe.rotatex(-params.tilt);
            worldframe.rotatez(-params.rotation);
        }
        worldframe += planet.loc;
        let viewframe = self.world_to_view(&worldframe);
        if viewframe.x <= 0.0 { return; }
        let (screenx, screeny) = self.view_to_screen(&viewframe);
        
        if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
            if viewframe.x > self.buffer.depth[idx] { return; }
            self.buffer.set(idx, color, viewframe.x, None);
        }
    }

    fn render_planet(&mut self, planet: &Planet) {
        if self.behind_view(&planet.loc) { return; }
        let distance = self.distance_square(&planet.loc).sqrt() - planet.rad;
        let delta = (distance / (planet.rad * 200.0)).max(0.0075);
        let (thetadelta, phidelta) = (delta, delta * 2.0);
        let thetastep = (TAU / thetadelta) as Int;
        let phistep = (PI / phidelta) as Int;

        for thetamul in 0..thetastep {
            let theta = thetamul as Float * thetadelta;
            for phimul in 0..phistep {
                let phi = phimul as Float * phidelta;

                let (sint, cost) = theta.sin_cos();
                let (sinp, cosp) = phi.sin_cos();

                let spherex = planet.rad * cost * sinp;
                let spherey = planet.rad * sint * sinp;
                let spherez = planet.rad * cosp;
                let mut worldframe = Vec3::cons(spherex, spherey, spherez);
                if let Some(params) = &planet.params {
                    self.apply_param_transformations(&mut worldframe, params);
                }
                worldframe += planet.loc;

                let viewframe = self.world_to_view(&worldframe);
                if viewframe.x <= 0.0 { continue; }
                let (screenx, screeny) = self.view_to_screen(&viewframe);

                if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
                    if viewframe.x > self.buffer.depth[idx] { continue; }
                    let mut normal = worldframe - planet.loc;
                    normal.normalize();
                    let luminance = self.body_luminance(planet, worldframe, normal);
                    let mut color = self.map_texture(theta, phi, planet);
                    color.lighting(luminance);
                    self.buffer.set(idx, color, viewframe.x, None);
                }
            }
        }
    }

    fn body_luminance(&mut self, planet: &Planet, worldframe: Vec3, normal: Vec3) -> f32 {
        if planet.lightsource { return 1.0 }
        self.generalize_luminance(worldframe, normal)
    }

    fn generalize_luminance(&mut self, worldframe: Vec3, normal: Vec3) -> f32 {
        self.system.lightsources.iter().map(|lightsource| {
            let mut light = *lightsource - worldframe;
            light.normalize();
            light.inner_prod(&normal).max(0.0)
        }).sum::<Float>().min(1.0)
    }

    fn apply_param_transformations(&self, worldframe: &mut Vec3, params: &PlanetParams) {
        worldframe.rotatez(-params.rotation);
        worldframe.rotatex(-params.tilt);
    }

    fn world_to_view(&self, worldframe: &Vec3) -> Vec3 {
        let mut viewframe = *worldframe - self.viewmodel.pos;
        viewframe.rotatez(-self.viewmodel.rot);
        viewframe.rotatey(self.viewmodel.tilt);
        viewframe
    }

    fn view_to_screen(&self, viewframe: &Vec3) -> (Int, Int) {
        let invx = 1.0 / viewframe.x;
        let (modx, mody) = (invx * self.config.fov() * 2.0, invx * self.config.fov());
        let screenx = (viewframe.y * modx + self.buffer.halfwidth() as Float) as Int;
        let screeny = (viewframe.z * mody + self.buffer.halfheight() as Float) as Int;
        (screenx, screeny)
    }

    fn behind_view(&self, point: &Vec3) -> bool {
        let viewframe = self.world_to_view(point);
        viewframe.x <= 0.0
    }

    fn map_texture(&self, theta: Float, phi: Float, planet: &Planet) -> Color {
        if let Some(tex) = &planet.texture {
            let tx = (theta / TAU * (tex.width-1) as Float) as usize;
            let ty = (phi / PI * (tex.height-1) as Float) as usize;
            tex.get(tx, ty)
        }
        else {
            Color::cons(0, 250, 250)
        }
    }

    fn map_texture_ring(&self, theta: Float, gamma: Float, ring: &Ring) -> Color {
        let tx = (gamma / ring.depth * (ring.texture.width-1) as Float) as usize;
        let ty = (theta / TAU * (ring.texture.height-1) as Float) as usize;
        ring.texture.get(tx, ty)
    }

    fn distance_square(&self, point: &Vec3) -> Float {
        let relative = *point - self.viewmodel.pos;
        relative.inner_prod(&relative)
    }
}

pub struct TextureData {
    pub height: usize, pub width: usize,
    pub texture: Vec<Color>,
}

impl TextureData {
    pub fn from(path: &str) -> TextureData {
        let file = read_to_string(path).unwrap_or_else(|_| {
            panic!("unable to load texture {}", path)
        });
        let mut texture = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for line in file.lines() {
            for col in line.split_whitespace().rev() {
                let color = Color::from_str(col);
                texture.push(color);
            }
            width = line.split_whitespace().count();
            height += 1;
        }
        TextureData { height, width, texture }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.texture[y * self.width + x]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8, pub green: u8, pub blue: u8,
}

impl Color {
    pub fn cons(r: u8, g: u8, b: u8) -> Color {
        Color { red: r, green: g, blue: b }
    }

    pub fn to_ansiback(self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.red, self.green, self.blue)
    }

    pub fn to_u32(self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    pub fn from_str(string: &str) -> Color {
        let mut rgb = string.split(';');
        if let (Some(r), Some(g), Some(b)) = (rgb.next(), rgb.next(), rgb.next()) {
            if let (Ok(r), Ok(g), Ok(b)) = (r.parse::<u8>(), g.parse::<u8>(), b.parse::<u8>()) {
                return Color::cons(r, g, b);
            }
        }
        Color::cons(0, 0, 0)
    }

    pub fn lighting(&mut self, lumin: Float) {
        let lumin = lumin.max(0.05);
        self.red = (self.red as Float * lumin) as u8;
        self.green = (self.green as Float * lumin) as u8;
        self.blue = (self.blue as Float * lumin) as u8;
    }
}

pub struct Buffer {
    pub height: Int, pub width: Int,
    visual: Vec<char>,
    color: Vec<Option<Color>>,
    depth: Vec<Float>,
}

impl Buffer {
    pub fn cons(height: Int, width: Int) -> Buffer {
        let (wi, he) = (width as usize, height as usize);
        debug_assert!(wi < 500 && he < 500);
        Buffer {
            height, width,
            visual: vec![' '; wi * he],
            color: vec![None; wi * he],
            depth: vec![Float::MAX; wi * he],
        }
    }

    pub fn debug(&self) -> Vec<u32> {
        let mut output = Vec::with_capacity(self.height as usize * self.width as usize);
        self.color.iter().for_each(|color| {
            if let Some(color) = color {
                output.push(color.to_u32());
            }
            else {
                output.push(0xFF000000);
            }
        });
        output
    }

    pub fn inboundsdex(&self, x: Int, y: Int) -> Option<usize> {
        let (height, width) = (self.height as usize, self.width as usize);
        let (x, y) = (x as usize, y as usize);
        if x < width && y < height {
            let ytrans = height-1 - y;
            Some(ytrans * width + x)
        }
        else {
            None
        }
    }

    pub fn set(&mut self, idx: usize, color: Color, depth: Float, visual: Option<char>) {
        self.color[idx] = Some(color);
        self.depth[idx] = depth;
        if let Some(chr) = visual {
            self.visual[idx] = chr;
        }
    }

    pub fn display(&self) {
        let mut string = String::new();
        print!("\x1b[H");
        self.visual.iter().enumerate().for_each(|(idx, ele)| {
            if let Some(color) = self.color[idx] {
                string.push_str(&color.to_ansiback());
            }
            if idx % self.width as usize != 0 {
                string.push(*ele);
            }
            else {
                string.push('\n')
            }
            string.push_str("\x1b[0m");
        });
        print!("{}", string);
        stdout().flush().unwrap();
    }

    const fn halfheight(&self) -> Int {
        self.height / 2
    }

    const fn halfwidth(&self) -> Int {
        self.width / 2
    }

    pub fn clear(&mut self) {
        self.visual.fill(' ');
        self.color.fill(None);
        self.depth.fill(Float::MAX);
    }
}
