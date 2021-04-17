use rand::rngs::ThreadRng;

use super::Material;
use crate::rand;
use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::vec3::{Color, Vec3};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self {
            index_of_refraction: 0.0,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let eta = if rec.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit = ray.direction().normalized();

        let cos_th = f64::min(Vec3::dot(-unit, rec.normal()), 1.0);
        let sin_th = f64::sqrt(1.0 - cos_th * cos_th);

        let cannot_refract = eta * sin_th > 1.0;

        let direction = if cannot_refract || reflectance(cos_th, eta) > rand(rng) {
            unit.reflect(rec.normal())
        } else {
            unit.refract(rec.normal(), eta)
        };

        let scattered = Ray::new(rec.point(), direction, ray.time());

        Some((scattered, attenuation))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
