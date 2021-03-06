use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
