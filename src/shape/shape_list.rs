use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::{HitRecord, Shape};
use crate::aabb::AABB;
use crate::ray::Ray;

pub struct ShapeList {
    pub shapes: Vec<Rc<dyn Shape>>,
}

impl ShapeList {
    pub fn new(shapes: Vec<Rc<dyn Shape>>) -> Self {
        Self { shapes }
    }

    pub fn add(&mut self, shape: Rc<dyn Shape>) {
        self.shapes.push(shape);
    }
}

impl Default for ShapeList {
    fn default() -> Self {
        Self { shapes: Vec::new() }
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord> {
        let mut rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_max;

        for shape in &self.shapes {
            if let Some(tmp) = shape.hit(ray, t_min, closest, rng) {
                hit_anything = true;
                closest = tmp.t;
                rec = tmp;
            }
        }

        if hit_anything {
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Rc<AABB>> {
        if self.shapes.is_empty() {
            return None;
        }

        let mut output_box = Rc::new(AABB::default());
        let mut first = true;
        for shape in &self.shapes {
            if let Some(tmp) = shape.bounding_box(time0, time1) {
                output_box = if first {
                    tmp
                } else {
                    Rc::new(AABB::surrounding_box(&output_box, &tmp))
                };
                first = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }
}
