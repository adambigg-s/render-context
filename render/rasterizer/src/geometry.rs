#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use std::fs::read_to_string;

use crate::math::Vec3;
use crate::{Color, Float};



#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Color,
}

impl Vertex {
    fn cons(pos: Vec3, color: Color) -> Vertex {
        Vertex { pos, color }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vertex, pub b: Vertex, pub c: Vertex,
}

impl Triangle {
    pub fn cons(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        Triangle { a: Vertex::cons(a, 0xffff0000), b: Vertex::cons(b, 0xff00ff00), c: Vertex::cons(c, 0xff0000ff) }
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
                    vertices.push(Vec3::cons(x, y, z));
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
                        tris.push(Triangle::cons(face_vertices[0], face_vertices[i-1], face_vertices[i]));
                    }
                }
                _ => {}
            }
        }

        Mesh::cons(tris, Vec3::cons(0, 0, 0))
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
    pub center: Vec3,
    pub length: Float,
}

impl RefFrame {
    pub fn cons(center: Vec3, length: Float) -> RefFrame {
        RefFrame { center, length }
    }
}

pub struct Gradient {
}

impl Gradient {
    
}
