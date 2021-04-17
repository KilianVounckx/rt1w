use rand::rngs::ThreadRng;
use std::rc::Rc;

use crate::aabb::AABB;
use crate::material::{lambertian::Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub mod bvh_node;
pub mod moving_sphere;
pub mod shape_list;
pub mod sphere;
pub mod xy_rect;
pub mod xz_rect;
pub mod yz_rect;
pub mod cube;
pub mod translate;
pub mod rotate_y;
pub mod constant_medium;

#[derive(Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: Rc<dyn Material>,
    u: f64,
    v: f64,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.material)
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            material: Rc::new(Lambertian::default()),
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Rc<AABB>>;
}
