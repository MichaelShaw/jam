extern crate cgmath;

use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SpringState3 {
    pub target: Vec3,
    pub position:Vec3,
    pub velocity:Vec3,
}

impl SpringState3 {
    pub fn new(at:Vec3) -> SpringState3 {
        SpringState3 {
            target: at,
            position: at,
            velocity: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn advance(&mut self, smooth_time:f64, time_delta: f64) {
        let (new_position, new_velocity) = smooth_3d(self.position, self.target, self.velocity, smooth_time, time_delta);
        self.position = new_position;
        self.velocity = new_velocity;
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SpringState1 {
    pub target: f64,
    pub position: f64,
    pub velocity: f64,    
}

impl SpringState1 {
    pub fn new(at:f64) -> SpringState1 {
        SpringState1 {
            target: at,
            position: at,
            velocity: 0.0,
        }
    }

    pub fn advance(&mut self, smooth_time:f64, time_delta: f64) {
        let (new_position, new_velocity) = smooth_1d(self.position, self.target, self.velocity, smooth_time, time_delta);
        self.position = new_position;
        self.velocity = new_velocity;
    }
}

pub fn smooth_3d(from:Vec3, to:Vec3, velocity:Vec3, smooth_time:f64, time_delta:f64) -> (Vec3, Vec3) {
    let omega = 2.0 / smooth_time;
    let x = omega * time_delta;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = from - to;

    let tmp = (velocity + change * omega) * time_delta;

    let new_velocity = (velocity - omega * tmp) * exp;

    (to + (change + tmp) * exp, new_velocity)
} 

pub fn smooth_1d(from:f64, to:f64, velocity:f64, smooth_time:f64, time_delta:f64) -> (f64, f64) {
    let omega = 2.0 / smooth_time;
    let x = omega * time_delta;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = from - to;

    let tmp = (velocity + omega * change) * time_delta;

    let new_velocity = (velocity - omega * tmp) * exp;

    (to + (change + tmp) * exp, new_velocity)
}