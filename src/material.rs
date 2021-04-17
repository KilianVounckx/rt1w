use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::vec3::{Color, Point3};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)>;

    fn emitted(&self, _: f64, _: f64, _: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
