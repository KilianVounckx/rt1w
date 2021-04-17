use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::{HitRecord, Shape};
use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    fn uv(&self, point: &Point3) -> (f64, f64) {
        use std::f64::consts::PI;
        let theta = f64::acos(-point.y());
        let phi = f64::atan2(-point.z(), point.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _: &mut ThreadRng) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
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
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Rc::clone(&self.material);
        let (u, v) = self.uv(&outward_normal);
        rec.u = u;
        rec.v = v;

        return Some(rec);
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Rc<AABB>> {
        Some(Rc::new(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        )))
    }
}
