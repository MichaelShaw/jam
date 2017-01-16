extern crate cgmath;

use Vec3;
use cgmath::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Line {
    pub from: Vec3,
    pub to: Vec3,
}

const EPSILON: f64 = 0.0000001;

impl Line {
    pub fn intersects(&self, plane:Plane) -> Option<Vec3> {
        let direction = (self.to - self.from).normalize();
        let denominator = dot(plane.normal, direction);

        if denominator > -EPSILON && denominator < EPSILON {
            return None
        }
        let numerator = -(dot(plane.normal, self.from) - plane.coefficient);
        let ratio = numerator / denominator;

        if ratio < EPSILON {
            None
        } else {
            Some((direction * ratio) + self.from)
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Plane {
    pub normal: Vec3,
    pub coefficient: f64,
}

impl Plane {
    pub fn from_origin_normal(origin:Vec3, normal:Vec3) -> Plane {
        Plane {
            normal:normal,
            coefficient: (normal.x * origin.x + normal.y * origin.y + normal.z * origin.z),
        }
    }
}