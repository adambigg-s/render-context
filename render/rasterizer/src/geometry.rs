#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use std::fs::read_to_string;

use crate::math::Vec3f;
use crate::{Color, Float};



#[derive(Clone, Copy)]
pub struct Vertf {
    pub pos: Vec3f,
    pub color: Color,
}

impl Vertf {
    pub fn cons(pos: Vec3f, color: Color) -> Vertf {
        Vertf { pos, color }
    }
}

#[derive(Clone, Copy)]
pub struct Trif {
    pub a: Vertf, pub b: Vertf, pub c: Vertf,
}

impl Trif {
    pub fn cons(a: Vec3f, b: Vec3f, c: Vec3f) -> Trif {
        Trif {
            a: Vertf::cons(a, Color::cons(255, 0, 0)),
            b: Vertf::cons(b, Color::cons(0, 255, 0)),
            c: Vertf::cons(c, Color::cons(0, 0, 255)),
        }
    }

    pub fn cons_verts(a: Vertf, b: Vertf, c: Vertf) -> Trif {
        Trif { a, b, c }
    }

    pub fn get_normal(&self) -> Vec3f {
        let mut normal = (self.a.pos - self.b.pos).cross(&(self.a.pos - self.c.pos));
        normal.normalize();
        normal
    }

    pub fn rotatex(&mut self, angle: Float) {
        self.a.pos.rotatex(angle);
        self.b.pos.rotatex(angle);
        self.c.pos.rotatex(angle);
    }
    
    pub fn rotatey(&mut self, angle: Float) {
        self.a.pos.rotatey(angle);
        self.b.pos.rotatey(angle);
        self.c.pos.rotatey(angle);
    }

    pub fn rotatez(&mut self, angle: Float) {
        self.a.pos.rotatez(angle);
        self.b.pos.rotatez(angle);
        self.c.pos.rotatez(angle);
    }

    pub fn rotatezyx(&mut self, angles: Vec3f) {
        self.rotatez(angles.z);
        self.rotatey(angles.y);
        self.rotatex(angles.x);
    }

    pub fn long_left(&self) -> bool {
        let v1 = self.a.pos - self.b.pos;
        let v2 = self.a.pos - self.c.pos;
        v1.x * v2.y - v1.y * v2.x <= 0.
    }
}

pub struct Mesh {
    pub tris: Vec<Trif>,
    pub center: Vec3f,
    pub rotation: Vec3f,
}

impl Mesh {
    pub fn cons(tris: Vec<Trif>, center: Vec3f) -> Mesh {
        Mesh { tris, center, rotation: Vec3f::cons(0, 0, 0) }
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
                    vertices.push(Vec3f::cons(x, y, z));
                }
                "f" => {
                    let i0: usize = parts[1].parse().unwrap();
                    let i1: usize = parts[2].parse().unwrap();
                    let i2: usize = parts[3].parse().unwrap();

                    tris.push(Trif::cons(vertices[i0-1], vertices[i1-1], vertices[i2-1]));
                }
                _ => {}
            }
        }

        Mesh::cons(tris, Vec3f::cons(0, 0, 0))
    }

    pub fn build_from_file_extended(path: &str, scaling: Float) -> Mesh {
        let data = read_to_string(path).unwrap();
        let mut vertices = Vec::new();
        let mut tris = Vec::new();

        for line in data.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() { continue };

            match parts[0] {
                "v" => {
                    let x: Float = parts[1].parse::<Float>().unwrap() * scaling;
                    let y: Float = parts[2].parse::<Float>().unwrap() * scaling;
                    let z: Float = parts[3].parse::<Float>().unwrap() * scaling;
                    vertices.push(Vec3f::cons(x, y, z));
                }
                "f" => {
                    let mut face_vertices = Vec::new();
                    for part in parts.iter().skip(1) {
                        let indices: Vec<&str> = part.split('/').collect();
                        if let Ok(index) = indices[0].parse::<usize>() {
                            face_vertices.push(vertices[index-1]);
                        }
                    }
                    for i in 2..face_vertices.len() {
                        tris.push(Trif::cons(face_vertices[0], face_vertices[i-1], face_vertices[i]));
                    }
                }
                _ => {}
            }
        }

        Mesh::cons(tris, Vec3f::cons(0, 0, 0))
    }

    pub fn rotatex(&mut self, angle: Float) {
        self.rotation.x += angle;
    }

    pub fn rotatey(&mut self, angle: Float) {
        self.rotation.y += angle;
    }

    pub fn rotatez(&mut self, angle: Float) {
        self.rotation.z += angle;
    }
}

pub struct RefFrame {
    pub center: Vec3f,
    pub length: Float,
}

impl RefFrame {
    pub fn cons(center: Vec3f, length: Float) -> RefFrame {
        RefFrame { center, length }
    }
}

pub struct Gradient {
}

impl Gradient {
}
