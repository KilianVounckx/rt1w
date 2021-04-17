use rand::rngs::ThreadRng;
use std::rc::Rc;

use crate::material::{lambertian::Lambertian, Material};
use crate::shape::{shape_list::ShapeList, sphere::Sphere};
use crate::texture::perlin::Perlin;
use crate::vec3::Point3;

pub fn build(rng: &mut ThreadRng) -> ShapeList {
    let mut shapes = ShapeList::default();

    let perlin: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(Perlin::new(4.0, 7, rng))));

    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&perlin),
    )));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::clone(&perlin),
    )));

    shapes
}
