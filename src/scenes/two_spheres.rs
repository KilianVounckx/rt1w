use std::rc::Rc;

use crate::material::{lambertian::Lambertian, Material};
use crate::shape::{shape_list::ShapeList, sphere::Sphere};
use crate::texture::checkers::Checkers;
use crate::vec3::{Color, Point3};

pub fn build() -> ShapeList {
    let mut shapes = ShapeList::default();

    let checker: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(Checkers::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));

    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::clone(&checker),
    )));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::clone(&checker),
    )));

    shapes
}
