use rand::rngs::ThreadRng;
use std::rc::Rc;

use super::shape_list::ShapeList;
use super::xy_rect::XyRect;
use super::xz_rect::XzRect;
use super::yz_rect::YzRect;
use super::{HitRecord, Shape};

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Cube {
    minimum: Point3,
    maximum: Point3,
    sides: ShapeList,
}

impl Cube {
    pub fn new(minimum: Point3, maximum: Point3, material: Rc<dyn Material>) -> Self {
        let mut sides = ShapeList::default();
        sides.add(Rc::new(XyRect::new(
            minimum.x(),
            maximum.x(),
            minimum.y(),
            maximum.y(),
            maximum.z(),
            Rc::clone(&material),
        )));
        sides.add(Rc::new(XyRect::new(
            minimum.x(),
            maximum.x(),
            minimum.y(),
            maximum.y(),
            minimum.z(),
            Rc::clone(&material),
        )));

        sides.add(Rc::new(XzRect::new(
            minimum.x(),
            maximum.x(),
            minimum.z(),
            maximum.z(),
            maximum.y(),
            Rc::clone(&material),
        )));
        sides.add(Rc::new(XzRect::new(
            minimum.x(),
            maximum.x(),
            minimum.z(),
            maximum.z(),
            minimum.y(),
            Rc::clone(&material),
        )));

        sides.add(Rc::new(YzRect::new(
            minimum.y(),
            maximum.y(),
            minimum.z(),
            maximum.z(),
            maximum.x(),
            Rc::clone(&material),
        )));
        sides.add(Rc::new(YzRect::new(
            minimum.y(),
            maximum.y(),
            minimum.z(),
            maximum.z(),
            minimum.x(),
            Rc::clone(&material),
        )));
        Self {
            minimum,
            maximum,
            sides,
        }
    }
}

impl Shape for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max, rng)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Rc<AABB>> {
        Some(Rc::new(AABB::new(self.minimum, self.maximum)))
    }
}
