use rand::rngs::ThreadRng;
use std::cmp::Ordering::{self, Greater, Less};
use std::rc::Rc;

use super::{shape_list::ShapeList, HitRecord, Shape};
use crate::aabb::AABB;
use crate::ray::Ray;

pub struct BvhNode {
    left: Rc<dyn Shape>,
    right: Rc<dyn Shape>,
    bounding_box: Rc<AABB>,
}

impl BvhNode {
    pub fn new(list: ShapeList, time0: f64, time1: f64, rng: &mut ThreadRng) -> Self {
        Self::new_from_vec(&mut list.shapes.to_vec(), time0, time1, rng)
    }

    fn new_from_vec(
        objects: &mut [Rc<dyn Shape>],
        time0: f64,
        time1: f64,
        rng: &mut ThreadRng,
    ) -> Self {
        let axis = crate::rand_int(0, 3, rng);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let span = objects.len();
        let (left, right): (Rc<dyn Shape>, Rc<dyn Shape>) = if span == 1 {
            (Rc::clone(&objects[0]), Rc::clone(&objects[0]))
        } else if span == 2 {
            if comparator(&objects[0], &objects[1]) == Less {
                (Rc::clone(&objects[0]), Rc::clone(&objects[1]))
            } else {
                (Rc::clone(&objects[1]), Rc::clone(&objects[0]))
            }
        } else {
            objects.sort_unstable_by(comparator);

            let mid = span / 2;
            let (left_objects, right_objects) = objects.split_at_mut(mid);
            (
                Rc::new(Self::new_from_vec(left_objects, time0, time1, rng)),
                Rc::new(Self::new_from_vec(right_objects, time0, time1, rng)),
            )
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        let bounding_box = match (box_left, box_right) {
            (Some(left), Some(right)) => Rc::new(AABB::surrounding_box(&left, &right)),
            _ => panic!("No bounding box in BvhNode constructor.\n"),
        };

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

fn box_compare(a: &Rc<dyn Shape>, b: &Rc<dyn Shape>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 1.0);
    let box_b = b.bounding_box(0.0, 1.0);

    match (box_a, box_b) {
        (Some(a), Some(b)) => {
            if a.minimum()[axis] < b.minimum()[axis] {
                Less
            } else {
                Greater
            }
        }
        _ => panic!("No bounding box in BvhNode constructor.\n"),
    }
}

fn box_x_compare(a: &Rc<dyn Shape>, b: &Rc<dyn Shape>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Rc<dyn Shape>, b: &Rc<dyn Shape>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Rc<dyn Shape>, b: &Rc<dyn Shape>) -> Ordering {
    box_compare(a, b, 2)
}

impl Shape for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut ThreadRng) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rng);
        if let Some(rec1) = hit_left {
            if let Some(rec2) = self.right.hit(ray, t_min, rec1.t, rng) {
                Some(rec2)
            } else {
                Some(rec1)
            }
        } else {
            if let Some(rec2) = self.right.hit(ray, t_min, t_max, rng) {
                Some(rec2)
            } else {
                None
            }
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Rc<AABB>> {
        Some(Rc::clone(&self.bounding_box))
    }
}
