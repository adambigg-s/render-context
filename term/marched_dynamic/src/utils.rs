#![allow(dead_code, unused_variables)]



use crate::{math::Vec3, Float};



pub struct Ball {
    position: Vec3,
    velocity: Vec3,
    collider: BoxCollider,
}

pub struct SphereCollider {
    center: Vec3,
    radius: Float,
}

pub struct BoxCollider {
    xmax: Float,
    xmin: Float,
    ymax: Float,
    ymin: Float,
    zmax: Float,
    zmin: Float,
}

impl BoxCollider {
    pub fn no_collision_self(&self, other: &Self) -> bool {
        (self.xmax < other.xmin || self.xmin > other.xmax) &&
        (self.ymax < other.ymin || self.ymin > other.ymax) &&
        (self.zmax < other.zmin || self.zmin > other.zmin)
    }

    pub fn no_collision_sphere(&self, other: &SphereCollider) -> bool {
        false
    }
}
