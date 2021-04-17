use rand::rngs::ThreadRng;

use super::Material;
use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::vec3::{Color, Vec3};

pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self { color, fuzz }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            color: Color::default(),
            fuzz: 0.0,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let reflected = ray.direction().reflect(rec.normal());
        let scattered = Ray::new(
            rec.point(),
            reflected + self.fuzz * Vec3::rand_in_unit_sphere(rng),
            ray.time(),
        );
        let attenuation = self.color;
        Some((scattered, attenuation))
    }
}
