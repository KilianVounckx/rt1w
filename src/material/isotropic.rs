use rand::rngs::ThreadRng;
use std::rc::Rc;

use crate::texture::{Texture, solid_color::SolidColor};
use crate::vec3::{Color, Vec3};
use crate::ray::Ray;
use crate::shape::HitRecord;
use super::Material;

pub struct Isotropic {
	texture: Rc<dyn Texture>,
}

impl Isotropic {
	pub fn new(texture: Rc<dyn Texture>) -> Self {
		Self {texture}
	}
	pub fn from_color(color: Color) -> Self {
		Self {
			texture: Rc::new(SolidColor::new(color)),
		}
	}
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Color)> {
		let scattered = Ray::new(rec.point(), Vec3::rand_in_unit_sphere(rng), ray.time());
		let color = self.texture.color(rec.u(), rec.v(), rec.point());
		Some((scattered, color))
	}
}
