#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use std::fs::read_to_string;
use std::mem::swap;

use crate::{Color, Float, Int};
use crate::texture::Texture;
use crate::math::{Vec2f, Vec3f};



pub struct PolyData {
    pub tri: Tri,
    pub normal: Vec3f,
    pub lighting: Float,
}

impl PolyData {
    pub fn cons(tri: Tri, normal: Vec3f, lighting: Float) -> PolyData {
        PolyData { tri, normal, lighting }
    }
}



#[derive(Clone, Copy)]
pub struct Vert {
    pub pos: Vec3f,
    pub texpos: Vec2f,
    pub color: Color,
}

impl Vert {
    pub fn cons(pos: Vec3f, color: Color, texpos: Vec2f) -> Vert {
        Vert { pos, color, texpos }
    }
}



#[derive(Clone, Copy)]
pub struct Tri {
    pub a: Vert, pub b: Vert, pub c: Vert,
}

impl Tri {
    pub fn cons_pos(a: Vec3f, b: Vec3f, c: Vec3f) -> Tri {
        Tri {
            a: Vert::cons(a, Color::cons(255, 0, 0), Vec2f::cons(0, 0)),
            b: Vert::cons(b, Color::cons(0, 255, 0), Vec2f::cons(0, 0)),
            c: Vert::cons(c, Color::cons(0, 0, 255), Vec2f::cons(0, 0)),
        }
    }

    pub fn cons_vert(a: Vert, b: Vert, c: Vert) -> Tri {
        Tri { a, b, c }
    }

    pub fn sort_verts_vertical(&mut self) {
        if self.c.pos.y > self.b.pos.y {
            // (self.c, self.b) = (self.b, self.c);
            swap(&mut self.c, &mut self.b);
        }
        if self.b.pos.y > self.a.pos.y {
            // (self.a, self.b) = (self.b, self.a);
            swap(&mut self.b, &mut self.a);
        }
        if self.c.pos.y > self.b.pos.y {
            // (self.c, self.b) = (self.b, self.c);
            swap(&mut self.c, &mut self.b);
        }
        {
            debug_assert!(self.a.pos.y >= self.b.pos.y && self.b.pos.y >= self.c.pos.y);
        }
    }

    pub fn get_red_ordered_vec(&self) -> Vec3f {
        Vec3f::cons(self.a.color.red, self.b.color.red, self.c.color.red)
    }
    
    pub fn get_green_ordered_vec(&self) -> Vec3f {
        Vec3f::cons(self.a.color.green, self.b.color.green, self.c.color.green)
    }

    pub fn get_blue_ordered_vec(&self) -> Vec3f {
        Vec3f::cons(self.a.color.blue, self.b.color.blue, self.c.color.blue)
    }

    pub fn get_normal(&self) -> Vec3f {
        (self.a.pos - self.b.pos).cross(&(self.a.pos - self.c.pos)).get_normalized()
    }

    pub fn interpolate_depth_linear(&self, weights: Vec3f) -> Float {
        let depths = Vec3f::cons(self.a.pos.z, self.b.pos.z, self.c.pos.z);
        depths.inner_prod(&weights)
    }

    pub fn interpolate_depth_nonlinear(&self, weights: Vec3f) -> Float {
        let depths = Vec3f::cons(1. / self.a.pos.z, 1. / self.b.pos.z, 1. / self.c.pos.z);
        1. / depths.inner_prod(&weights)
    }
    
    pub fn interpolate_tex_u(&self, coords: &Vec3f) -> Float {
        Vec3f::cons(self.a.texpos.x, self.b.texpos.x, self.c.texpos.x)
            .inner_prod(coords)
    }

    pub fn interpolate_tex_v(&self, coords: &Vec3f) -> Float {
        Vec3f::cons(self.a.texpos.y, self.b.texpos.y, self.c.texpos.y)
            .inner_prod(coords)
    }

    pub fn rot_x(&mut self, angle: Float) {
        self.a.pos.rot_x(angle);
        self.b.pos.rot_x(angle);
        self.c.pos.rot_x(angle);
    }
    
    pub fn rot_y(&mut self, angle: Float) {
        self.a.pos.rot_y(angle);
        self.b.pos.rot_y(angle);
        self.c.pos.rot_y(angle);
    }

    pub fn rot_z(&mut self, angle: Float) {
        self.a.pos.rot_z(angle);
        self.b.pos.rot_z(angle);
        self.c.pos.rot_z(angle);
    }

    pub fn rot_xyz(&mut self, angles: Vec3f) {
        self.rot_x(angles.x);
        self.rot_y(angles.y);
        self.rot_z(angles.z);
    }

    pub fn rot_zyx(&mut self, angles: Vec3f) {
        self.rot_z(angles.z);
        self.rot_y(angles.y);
        self.rot_x(angles.x);
    }

    pub fn translate(&mut self, vec: Vec3f) {
        self.a.pos += vec;
        self.b.pos += vec;
        self.c.pos += vec;
    }

    pub fn lumped_left(&self) -> bool {
        let v1 = self.a.pos - self.b.pos;
        let v2 = self.a.pos - self.c.pos;
        v1.x * v2.y - v1.y * v2.x <= 0.
    }

    pub fn lumped_right(&self) -> bool {
        let v1 = self.a.pos - self.b.pos;
        let v2 = self.a.pos - self.c.pos;
        v1.x * v2.y - v1.y * v2.x >= 0.
    }
}



pub struct Mesh {
    pub tris: Vec<Tri>,
    pub center: Vec3f,
    pub rotation: Vec3f,
    pub texture: Option<Texture>,
}

impl Mesh {
    pub fn cons(tris: Vec<Tri>, center: Vec3f, texpath: Option<&str>) -> Mesh {
        Mesh { tris, center, rotation: Vec3f::cons(0, 0, 0), texture: texpath.map(Texture::build_from_file) }
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

                    tris.push(Tri::cons_pos(vertices[i0-1], vertices[i1-1], vertices[i2-1]));
                }
                _ => {}
            }
        }

        Mesh::cons(tris, Vec3f::cons(0, 0, 0), None)
    }

    pub fn build_from_file_extended(path: &str, scaling: Float, texpath: Option<&str>) -> Mesh {
        let data = read_to_string(path).unwrap();
        let mut vertices = Vec::new();
        let mut tris = Vec::new();
        let mut tex_coords = Vec::new();

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
                "vt" => {
                    let u: Float = parts[1].parse().unwrap();
                    let v: Float = parts[2].parse::<Float>().unwrap() * -1. + 1.;
                    tex_coords.push(Vec2f::cons(u, v));
                }
                "f" => {
                    let mut face_vertices = Vec::new();
                    for part in parts.iter().skip(1) {
                        let indices: Vec<&str> = part.split('/').collect();
                        let vert_idx = indices[0].parse::<usize>().unwrap() - 1;
                        let vert = vertices[vert_idx];

                        let tex_coord = if indices.len() > 1 && !indices[1].is_empty() {
                            let tex_idx = indices[1].parse::<usize>().unwrap() - 1;
                            tex_coords[tex_idx]
                        }
                        else {
                            Vec2f::cons(0, 0,)
                        };
                        face_vertices.push((vert, tex_coord));
                    }

                    for i in 2..face_vertices.len() {
                        let (v0, t0) = face_vertices[0];
                        let (v1, t1) = face_vertices[i - 1];
                        let (v2, t2) = face_vertices[i];

                        let v0 = Vert::cons(v0, Color::cons(255, 255, 255), t0);
                        let v1 = Vert::cons(v1, Color::cons(255, 255, 255), t1);
                        let v2 = Vert::cons(v2, Color::cons(255, 255, 255), t2);

                        tris.push(Tri::cons_vert(v0, v1, v2));
                    }
                }
                _ => {}
            }
        }

        Mesh::cons(tris, Vec3f::cons(0, 0, 0), texpath)
    }

    pub fn rotate_x(&mut self, angle: Float) {
        self.rotation.x += angle;
    }

    pub fn rotate_y(&mut self, angle: Float) {
        self.rotation.y += angle;
    }

    pub fn rotate_z(&mut self, angle: Float) {
        self.rotation.z += angle;
    }
}



pub struct BarycentricSystem<'d> {
    triangle: &'d Tri,
    a: Vec3f,
    b: Vec3f,
    c: Vec3f,
    inv_den: Float,
}

impl BarycentricSystem<'_> {
    pub fn cons(triangle: &Tri) -> BarycentricSystem {
        let a = triangle.a.pos;
        let b = triangle.b.pos;
        let c = triangle.c.pos;
        let den = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
        let inv_den = 1. / den;
        
        BarycentricSystem { triangle, a, b, c, inv_den }
    }

    pub fn get_coords(&self, x: Int, y: Int) -> Vec3f {
        let x = x as Float;
        let y = y as Float;
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let w1 = ((b.y - c.y) * (x - c.x) + (c.x - b.x) * (y - c.y)) * self.inv_den;
        let w2 = ((c.y - a.y) * (x - c.x) + (a.x - c.x) * (y - c.y)) * self.inv_den;
        Vec3f::cons(w1, w2, 1. - w1 - w2)
    }
}



#[derive(Clone, Copy)]
pub struct RefFrame {
    pub center: Vec3f,
    pub length: Float,
}

impl RefFrame {
    pub fn cons(center: Vec3f, length: Float) -> RefFrame {
        RefFrame { center, length }
    }

    pub fn translate(&mut self, translation: Vec3f) {
        self.center += translation;
    }
}
