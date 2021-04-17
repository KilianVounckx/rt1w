use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::{Shape, HitRecord};
use crate::material::{Material, isotropic::Isotropic};
use crate::texture::Texture;
use crate::vec3::{Vec3, Color};
use crate::ray::Ray;
use crate::aabb::AABB;

pub struct ConstantMedium {
	boundary: Rc<dyn Shape>,
	material: Rc<dyn Material>,
	neg_inv_density: f64,
}

impl ConstantMedium {
	pub fn new(boundary: Rc<dyn Shape>, density: f64, texture: Rc<dyn Texture>) -> Self {
		Self {
			boundary,
			neg_inv_density: -1.0 / density,
			material: Rc::new(Isotropic::new(texture)),
		}
	}
	pub fn from_color(boundary: Rc<dyn Shape>, density: f64, color: Color) -> Self {
		Self {
			boundary,
			neg_inv_density: -1.0 / density,
			material: Rc::new(Isotropic::from_color(color)),
		}
	}
}

impl Shape for ConstantMedium {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord> {
		let enable_debug = false;
		let debugging = enable_debug && crate::rand(rng) < 0.000_01;

		let mut rec1 = match self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY, rng) {
			Some(rec) => rec,
			None => return None,
		};
		let mut rec2 = match self.boundary.hit(ray, rec1.t + 0.0001, f64::INFINITY, rng) {
			Some(rec) => rec,
			None => return None,
		};

		if debugging {
			eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
		}

		if rec1.t < t_min {
			rec1.t = t_min;
		}
		if rec2.t > t_max {
			rec2.t = t_max;
		}

		if rec1.t >= rec2.t {
			return None;
		}

		if rec1.t < 0.0 {
			rec1.t = 0.0;
		}

		let ray_length = ray.direction().norm();
		let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
		let hit_distance = self.neg_inv_density * crate::rand(rng).ln();

		if hit_distance > distance_inside_boundary {
			return None;
		}

		let mut rec = HitRecord::default();
		rec.t = rec1.t + hit_distance / ray_length;
		rec.point = ray.at(rec.t);

		if debugging {
			eprintln!("hit_distance = {}", hit_distance);
			eprintln!("rec.t = {}", rec.t);
			eprintln!("rec.point = {}", rec.point);
		}

		rec.normal = Vec3::new(1.0, 0.0, 0.0);
		rec.material = Rc::clone(&self.material);

		Some(rec)
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Rc<AABB>> {
		self.boundary.bounding_box(time0, time1)
	}
}
