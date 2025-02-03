#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use std::fs::read_to_string;

use crate::{math::Vec3, Float};



pub struct Triangle {
    pub a: Vec3, pub b: Vec3, pub c: Vec3,
}

impl Triangle {
    pub fn cons(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        Triangle { a, b, c }
    }

    pub fn rotatex(&mut self, angle: Float) {
        self.a.rotatex(angle);
        self.b.rotatex(angle);
        self.c.rotatex(angle);
    }
    
    pub fn rotatey(&mut self, angle: Float) {
        self.a.rotatey(angle);
        self.b.rotatey(angle);
        self.c.rotatey(angle);
    }

    pub fn rotatez(&mut self, angle: Float) {
        self.a.rotatez(angle);
        self.b.rotatez(angle);
        self.c.rotatez(angle);
    }

    pub fn rotatezyx(&mut self, angles: Vec3) {
        self.rotatez(angles.z);
        self.rotatey(angles.y);
        self.rotatex(angles.x);
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
    pub center: Vec3,
    pub rotation: Vec3,
}

impl Mesh {
    pub fn cons(tris: Vec<Triangle>, center: Vec3) -> Mesh {
        Mesh { tris, center, rotation: Vec3::cons(0, 0, 0) }
    }

    pub fn build_from_file(path: &str, scaling: Float) -> Mesh {
        let data = read_to_string(path).unwrap();
        let mut vertices = Vec::new();
        let mut tris = Vec::new();

        for line in data.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() { continue; }

            match parts[0] {
                "v" => {
                    let x: Float = parts[1].parse::<Float>().unwrap() * scaling;
                    let y: Float = parts[2].parse::<Float>().unwrap() * scaling;
                    let z: Float = parts[3].parse::<Float>().unwrap() * scaling;
                    vertices.push(Vec3::cons(x, y, z));
                }
                "f" => {
                    let i0: usize = parts[1].parse().unwrap();
                    let i1: usize = parts[2].parse().unwrap();
                    let i2: usize = parts[3].parse().unwrap();

                    tris.push(Triangle::cons(vertices[i0-1], vertices[i1-1], vertices[i2-1]));
                }
                _ => {}
            }
        }

        Mesh::cons(tris, Vec3::cons(0, 0, 0))
    }

    pub fn rotatex(&mut self, angle: Float) {
        self.rotation.x += angle;
        self.tris.iter_mut().for_each(|tri| {
            tri.rotatex(angle);
        });
    }

    pub fn rotatey(&mut self, angle: Float) {
        self.rotation.y += angle;
        self.tris.iter_mut().for_each(|tri| {
            tri.rotatey(angle);
        });
    }

    pub fn rotatez(&mut self, angle: Float) {
        self.rotation.z += angle;
        self.tris.iter_mut().for_each(|tri| {
            tri.rotatez(angle);
        });
    }
}
