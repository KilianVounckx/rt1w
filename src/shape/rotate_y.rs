use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::{HitRecord, Shape};
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct RotateY {
    shape: Rc<dyn Shape>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: Option<Rc<AABB>>,
}

impl RotateY {
    pub fn new(shape: Rc<dyn Shape>, angle: f64) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let aabb = match shape.bounding_box(0.0, 1.0) {
            None => None,
            Some(aabb) => {
                let mut minimum = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut maximum =
                    Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let i = i as f64;
                            let j = j as f64;
                            let k = k as f64;
                            let x = i * aabb.maximum().x() + (1.0 - i) * aabb.minimum().x();
                            let y = j * aabb.maximum().y() + (1.0 - j) * aabb.minimum().y();
                            let z = k * aabb.maximum().z() + (1.0 - k) * aabb.minimum().z();

                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;

                            let tester = Vec3::new(new_x, y, new_z);

                            for c in 0..3 {
                                minimum[c] = f64::min(minimum[c], tester[c]);
                                maximum[c] = f64::max(maximum[c], tester[c]);
                            }
                        }
                    }
                }
                Some(Rc::new(AABB::new(minimum, maximum)))
            }
        };

        Self {
            shape,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}

impl Shape for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated = Ray::new(origin, direction, ray.time());

        let mut rec = match self.shape.hit(&rotated, t_min, t_max, rng) {
            Some(rec) => rec,
            None => return None,
        };

        let mut point = rec.point();
        let mut normal = rec.normal();

        point[0] = self.cos_theta * rec.point()[0] + self.sin_theta * rec.point()[2];
        point[2] = -self.sin_theta * rec.point()[0] + self.cos_theta * rec.point()[2];

        normal[0] = self.cos_theta * rec.normal()[0] + self.sin_theta * rec.normal()[2];
        normal[2] = -self.sin_theta * rec.normal()[0] + self.cos_theta * rec.normal()[2];

        rec.point = point;
        rec.set_face_normal(&rotated, normal);

        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Rc<AABB>> {
        match &self.aabb {
            Some(aabb) => Some(Rc::clone(&aabb)),
            None => None,
        }
    }
}
