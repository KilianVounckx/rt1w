use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::Material;
use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::texture::{solid_color::SolidColor, Texture};
use crate::vec3::{Color, Vec3};

pub struct Lambertian {
    texture: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(color)),
        }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            texture: Rc::new(SolidColor::default()),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal() + Vec3::rand_unit(rng);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        let scattered = Ray::new(rec.point(), scatter_direction, ray.time());
        let attenuation = self.texture.color(rec.u(), rec.v(), rec.point());
        Some((scattered, attenuation))
    }
}
