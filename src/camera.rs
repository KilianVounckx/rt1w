use rand::rngs::ThreadRng;

use crate::rand_between;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct CameraConfig {
    pub from: Point3,
    pub at: Point3,
    pub up: Vec3,
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            from: Point3::new(1.0, 0.0, 0.0),
            at: Point3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            vfov: 40.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus: 1.0,
            time0: 0.0,
            time1: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(conf: CameraConfig) -> Self {
        let th = conf.vfov.to_radians();
        let h = f64::tan(th / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = conf.aspect_ratio * viewport_height;

        let w = (conf.from - conf.at).normalized();
        let u = Vec3::cross(conf.up, w).normalized();
        let v = Vec3::cross(w, u);

        let origin = conf.from;
        let horizontal = conf.focus * viewport_width * u;
        let vertical = conf.focus * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - conf.focus * w;

        let lens_radius = conf.aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
            time0: conf.time0,
            time1: conf.time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            rand_between(self.time0, self.time1, rng),
        )
    }
}
