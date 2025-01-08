


use std::fs::read_to_string;
use std::io::{stdout, Write};

use crate::entities::{Ellipse, SpacialReference};
use crate::{Float, Int, Sphere, System, ViewModel, ASCIIGRAD, PI, TAU};
use crate::math::Vec3;



pub struct Renderer<'d> {
    pub viewmodel: &'d ViewModel,
    pub buffer: &'d mut Buffer,
    pub system: &'d System,
    pub globals: &'d TextureGlobal<'d>,
}

impl<'d> Renderer<'d> {
    pub fn cons(
        view: &'d ViewModel, buff: &'d mut Buffer, sys: &'d System, globals: &'d TextureGlobal
    ) -> Renderer<'d> {
        Renderer { viewmodel: view, buffer: buff, system: sys, globals }
    }

    pub fn render_spheres(&mut self) {
        for sphere in &self.system.spheres {
            self.render_sphere(sphere);
        }
    }

    pub fn render_ellipses(&mut self) {
        for ellipse in &self.system.ellipses {
            self.render_ellipse(ellipse);
        }
    }

    pub fn render_spacerefs(&mut self) {
        for spaceref in &self.system.spacerefs {
            self.render_spaceref(spaceref);
        }
    }

    fn render_ellipse(&mut self, ellipse: &Ellipse) {
        let distance = self.distance_square(&ellipse.loc).sqrt();
        if self.behind_view(&ellipse.loc) || distance > 100.0 { return; }
        let thetadelta = (distance / (ellipse.semimajor * 170.0)).max(0.01);
        let thetastep = (TAU / thetadelta) as Int;

        for thetamul in 0..thetastep {
            let theta = thetamul as Float * thetadelta;

            let rad = ellipse.semimajor
                * (1.0 - ellipse.eccentricity * ellipse.eccentricity)
                / (1.0 + ellipse.eccentricity * theta.cos());

            let x = rad * theta.cos();
            let y = rad * theta.sin();
            let worldframe = Vec3::cons(x, y, 0.0) + ellipse.loc;

            let viewframe = self.world_to_view(&worldframe);
            if viewframe.x <= 0.0 { continue; }

            let (screenx, screeny) = self.view_to_screen(&viewframe);

            if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
                if viewframe.x > self.buffer.depth[idx] { continue; }
                let mut normal = worldframe - ellipse.loc;
                normal.normalize();
                let luminance = {
                    self.globals.lightsources.iter().map(|lightsource| {
                        let mut light = *lightsource - worldframe;
                        light.normalize();
                        light.dot(&normal).max(0.0)
                    }).sum::<Float>().min(1.0)
                };
                let mut color = Color::cons(204, 174, 6);
                color.lighting(luminance);
                self.buffer.color[idx] = Some(color);
                self.buffer.depth[idx] = viewframe.x;
            }

        }
    }

    fn render_spaceref(&mut self, spaceref: &SpacialReference) {
        let distance = self.distance_square(&spaceref.loc).sqrt();
        if self.behind_view(&spaceref.loc) || distance > 100.0 { return; }
        let delta = 1.0 / 2.0;
        let deltastep = (spaceref.length / delta) as Int;
        for deltamul in 0..deltastep {
            let axisdelta = deltamul as Float * delta;
            self.axis_assistant(spaceref, Vec3::cons(axisdelta, 0.0, 0.0), Color::cons(255, 10, 10));
            self.axis_assistant(spaceref, Vec3::cons(0.0, axisdelta, 0.0), Color::cons(10, 255, 10));
            self.axis_assistant(spaceref, Vec3::cons(0.0, 0.0, axisdelta), Color::cons(10, 10, 255));
        }
    }

    fn axis_assistant(&mut self, spaceref: &SpacialReference, delta: Vec3, color: Color) {
        let worldframe = spaceref.loc + delta;
        let viewframe = self.world_to_view(&worldframe);
        if viewframe.x <= 0.0 { return; }

        let (screenx, screeny) = self.view_to_screen(&viewframe);
        if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
            if viewframe.x > self.buffer.depth[idx] { return; }
            self.buffer.color[idx] = Some(color);
            self.buffer.depth[idx] = viewframe.x;
        }
    }

    fn render_sphere(&mut self, sphere: &Sphere) {
        if self.behind_view(&sphere.loc) { return; }
        
        let distance = self.distance_square(&sphere.loc).sqrt() - sphere.rad;
        let delta = (distance / (sphere.rad * 200.0)).max(0.0075);
        let (thetadelta, phidelta) = (delta, delta * 2.0);
        let thetastep = (TAU / thetadelta) as Int;
        let phistep = (PI / phidelta) as Int;

        for thetamul in 0..thetastep {
            let theta = thetamul as Float * thetadelta;
            for phimul in 0..phistep {
                let phi = phimul as Float * phidelta;

                let (sint, cost) = theta.sin_cos();
                let (sinp, cosp) = phi.sin_cos();

                let spherex = sphere.rad * cost * sinp + sphere.loc.x;
                let spherey = sphere.rad * sint * sinp + sphere.loc.y;
                let spherez = sphere.rad * cosp + sphere.loc.z;
                let worldframe = Vec3::cons(spherex, spherey, spherez);

                let viewframe = self.world_to_view(&worldframe);
                if viewframe.x <= 0.0 { continue; }

                let (screenx, screeny) = self.view_to_screen(&viewframe);

                if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
                    if viewframe.x > self.buffer.depth[idx] { continue; }
                    let mut normal = worldframe - sphere.loc;
                    normal.normalize();
                    let luminance = {
                        if !sphere.lightsource {
                            self.globals.lightsources.iter().map(|lightsource| {
                                let mut light = *lightsource - worldframe;
                                light.normalize();
                                light.dot(&normal).max(0.0)
                            }).sum::<Float>().min(1.0)
                        }
                        else {
                            1.0
                        }
                    };
                    let mut color = self.map_texture(theta, phi, sphere);
                    color.lighting(luminance);
                    self.buffer.color[idx] = Some(color);
                    self.buffer.depth[idx] = viewframe.x;
                }
            }
        }
    }

    fn world_to_view(&self, worldframe: &Vec3) -> Vec3 {
        let mut viewframe = *worldframe - self.viewmodel.pos;
        viewframe.rotatez(-self.viewmodel.rot);
        viewframe.rotatey(self.viewmodel.tilt);
        viewframe
    }

    fn view_to_screen(&self, viewframe: &Vec3) -> (Int, Int) {
        let (scalingx, scalingy) = (100.0, 48.0);
        let invx = 1.0 / viewframe.x;
        let (modx, mody) = (invx * scalingx, invx * scalingy);
        let screenx = (viewframe.y * modx + self.buffer.halfwidth() as Float) as Int;
        let screeny = (viewframe.z * mody + self.buffer.halfheight() as Float) as Int;
        (screenx, screeny)
    }

    fn behind_view(&self, point: &Vec3) -> bool {
        let viewframe = self.world_to_view(point);
        viewframe.x <= 0.0
    }

    fn map_texture(&self, theta: Float, phi: Float, sphere: &Sphere) -> Color {
        if let Some(tex) = &sphere.texture {
            let xfrac = theta / TAU;
            let yfrac = phi / PI;
            let tx = (xfrac * (tex.width-1) as Float) as usize;
            let ty = (yfrac * (tex.height-1) as Float) as usize;
            tex.texture[ty * tex.width + tx]
        }
        else {
            sphere.color
        }
    }

    fn distance_square(&self, point: &Vec3) -> Float {
        let relative = *point - self.viewmodel.pos;
        relative.dot(&relative)
    }
}

pub struct TextureData {
    pub texture: Vec<Color>,
    pub height: usize, pub width: usize,
}

impl TextureData {
    pub fn from(path: &str) -> TextureData {
        let file = read_to_string(path).expect("unable to load texture");
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
        TextureData { texture, height, width }
    }
}

pub struct TextureGlobal<'a> {
    pub _asciigrad: Vec<char>,
    pub lightsources: Vec<Vec3>,
    pub _system: &'a System,
}

impl<'a> TextureGlobal<'a> {
    pub fn new(_system: &'a System) -> TextureGlobal<'a> {
        let _asciigrad = ASCIIGRAD.chars().collect();

        let mut lightsources = Vec::new();
        for planet in &_system.spheres {
            if planet.lightsource {
                lightsources.push(planet.loc);
            }
        }
        TextureGlobal { _asciigrad, lightsources, _system }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8, green: u8, blue: u8,
}

impl Color {
    pub fn cons(r: u8, g: u8, b: u8) -> Color {
        Color { red: r, green: g, blue: b }
    }

    pub fn to_ansiback(self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.red, self.green, self.blue)
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
    height: Int, width: Int,
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
            depth: vec![0.0; wi * he],
        }
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
        println!("{}", string);
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
