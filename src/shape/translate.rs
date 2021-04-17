use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::{Shape, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::aabb::AABB;

pub struct Translate {
	shape: Rc<dyn Shape>,
	offset: Vec3,
}

impl Translate {
	pub fn new(shape: Rc<dyn Shape>, offset: Vec3) -> Self {
		Self {shape, offset}
	}
}

impl Shape for Translate {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord> {
		let moved = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
		let mut rec = match self.shape.hit(&moved, t_min,t_max, rng) {
			Some(rec) => rec,
			None => return None,
		};
		rec.point += self.offset;
		rec.set_face_normal(&moved, rec.normal);
		Some(rec)
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<Rc<AABB>> {
		let bounding_box = match self.shape.bounding_box(time0, time1) {
			Some(bounding_box) => bounding_box,
			None => return None,
		};

		Some(Rc::new(AABB::new(
			bounding_box.minimum() + self.offset,
			bounding_box.maximum() + self.offset,
		)))
	}
}
