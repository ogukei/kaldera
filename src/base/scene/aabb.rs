#![allow(dead_code)]

use nalgebra_glm as glm;

pub struct AABB {
    min: glm::Vec3,
    max: glm::Vec3,
}

impl AABB {
    pub fn sphere(center: &glm::Vec3, radius: f32) -> Self {
        let radius = glm::vec3(radius, radius, radius);
        let min = center - radius;
        let max = center + radius;
        Self {
            min,
            max,
        }
    }
}

pub struct Xorshift64 {
    x: u64,
}

impl Xorshift64 {
    pub fn new() -> Self {
        Self { x: 88172645463325252, }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.x;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.x = x;
        x
    }

    // @see http://prng.di.unimi.it/
    // xoshiro / xoroshiro generators and the PRNG shootout, 
    // section "Generating uniform doubles in the unit interval"
    pub fn next_uniform(&mut self) -> f64 {
        let v = self.next();
        let v = (v >> 12) | 0x3ff0000000000000u64;
        f64::from_bits(v) - 1.0
    }

    pub fn next_uniform_f32(&mut self) -> f32 {
        self.next_uniform() as f32
    }
}
