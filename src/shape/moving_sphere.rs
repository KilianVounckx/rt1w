use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::{HitRecord, Shape};
use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    radius: f64,
    material: Rc<dyn Material>,
    time0: f64,
    time1: f64,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Shape for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _: &mut ThreadRng) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().norm_squared();
        let b = Vec3::dot(oc, ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;
        let d = b * b - a * c;

        if d < 0.0 {
            return None;
        }
        let sqrtd = d.sqrt();

        let mut root = (-b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Rc::clone(&self.material);

        return Some(rec);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Rc<AABB>> {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(Rc::new(AABB::surrounding_box(&box0, &box1)))
    }
}
