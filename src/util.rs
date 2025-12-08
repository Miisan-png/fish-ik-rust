use bevy::prelude::*;
use std::f32::consts::PI;

pub fn relative_angle_diff(a: f32, b: f32) -> f32 {
    let mut diff = b - a;
    while diff < -PI { diff += 2.0 * PI; }
    while diff > PI { diff -= 2.0 * PI; }
    diff
}

pub struct Chain {
    pub joints: Vec<Vec2>,
    pub angles: Vec<f32>,
    pub link_size: f32,
    pub angle_constraint: f32,
}

impl Chain {
    pub fn new(origin: Vec2, count: usize, link_size: f32, angle_constraint: f32) -> Self {
        let mut joints = Vec::with_capacity(count);
        let mut angles = Vec::with_capacity(count);
        for i in 0..count {
            joints.push(origin + Vec2::new(i as f32 * link_size, 0.0));
            angles.push(PI);
        }
        Self { joints, angles, link_size, angle_constraint }
    }

    pub fn resolve(&mut self, target: Vec2) {
        if self.joints.is_empty() { return; }
        
        let current_pos = target;
        self.joints[0] = current_pos;
        
        for i in 1..self.joints.len() {
            let prev = self.joints[i-1];
            let curr = self.joints[i];
            
            let diff = curr - prev;
            let desired_angle = diff.y.atan2(diff.x) + PI;
            
            let reference_angle = if i == 1 {
                 self.angles[0]
            } else {
                 self.angles[i-1]
            };

            let angle_diff = relative_angle_diff(reference_angle, desired_angle);
            let constrained_diff = angle_diff.clamp(-self.angle_constraint, self.angle_constraint);
            let final_angle = reference_angle + constrained_diff;

            let back_angle = final_angle + PI;
            let offset = Vec2::new(back_angle.cos(), back_angle.sin()) * self.link_size;
            self.joints[i] = prev + offset; 
            
            self.angles[i] = final_angle;
        }
        
        if self.joints.len() > 1 {
             let diff = self.joints[1] - self.joints[0];
             self.angles[0] = diff.y.atan2(diff.x) + PI; 
        }
    }
}