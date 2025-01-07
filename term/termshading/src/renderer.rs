


use crate::{math::Vec3, Buffer, Float, Int, Sphere, System, ViewModel, ASCIIGRAD, LIGHT, PI, TAU};



pub struct Renderer<'d> {
    viewmodel: &'d ViewModel,
    buffer: &'d mut Buffer,
    system: &'d System,
    asciigrad: Vec<char>,
    light: Vec3,
}

impl<'d> Renderer<'d> {
    pub fn cons(view: &'d ViewModel, buff: &'d mut Buffer, sys: &'d System) -> Renderer<'d> {
        let asciigrad = ASCIIGRAD.chars().collect();
        let mut light = Vec3::cons(LIGHT[0], LIGHT[1], LIGHT[2]);
        light.normalize();
        Renderer { viewmodel: view, buffer: buff, system: sys, asciigrad, light }
    }

    pub fn render_spheres(&mut self) {
        for sphere in &self.system.spheres {
            self.render_sphere(sphere);
        }
    }
    
    pub fn render_sphere(&mut self, sphere: &Sphere) {
        let (thetadelta, phidelta) = (0.01, 0.03);
        let (scalingx, scalingy) = (200.0, 100.0);
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

                let mut normal = worldframe - sphere.loc;
                normal.normalize();

                let mut viewframe = worldframe - self.viewmodel.pos;
                viewframe.rotatez(-self.viewmodel.rot);
                viewframe.rotatey(self.viewmodel.tilt);

                if viewframe.x <= 0.0 { continue; }

                let invx = 1.0 / viewframe.x;
                let (modifierx, modifiery) = (invx * scalingx, invx * scalingy);
                let screenx = (viewframe.y * modifierx + self.buffer.halfwidth() as Float) as Int;
                let screeny = (viewframe.z * modifiery + self.buffer.halfheight() as Float) as Int;

                if let Some(idx) = self.buffer.inboundsdex(screenx, screeny) {
                    if invx < self.buffer.depth[idx] { continue; }
                    self.buffer.visual[idx] = self.luminosity_char(&normal);
                    self.buffer.depth[idx] = invx;
                }
            }
        }
    }

    fn luminosity_char(&self, normal: &Vec3) -> char {
        let luminosity = self.light.dot(normal).clamp(0.0, 1.0);
        let idx = ((self.asciigrad.len()-1) as Float * luminosity).round() as usize;
        self.asciigrad[idx]
    }
}
