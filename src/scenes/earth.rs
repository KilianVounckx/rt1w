use std::rc::Rc;

use crate::material::{lambertian::Lambertian, Material};
use crate::shape::{shape_list::ShapeList, sphere::Sphere, Shape};
use crate::texture::{image::Image, Texture};
use crate::vec3::Point3;

pub fn build() -> ShapeList {
    let earth: Rc<dyn Texture> = Rc::new(Image::new("images/earthmap.jpg"));
    let earth: Rc<dyn Material> = Rc::new(Lambertian::new(earth));
    let earth: Rc<dyn Shape> = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth));
    ShapeList::new(vec![earth])
}
