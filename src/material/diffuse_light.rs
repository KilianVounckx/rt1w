use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::Material;
use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::texture::{solid_color::SolidColor, Texture};
use crate::vec3::{Color, Point3};

pub struct DiffuseLight {
    texture: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut ThreadRng) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: Point3) -> Color {
        self.texture.color(u, v, point)
    }
}
