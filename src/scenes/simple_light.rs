use rand::rngs::ThreadRng;
use std::rc::Rc;

use crate::material::{diffuse_light::DiffuseLight, lambertian::Lambertian, Material};
use crate::shape::{shape_list::ShapeList, sphere::Sphere, xy_rect::XyRect};
use crate::texture::{perlin::Perlin, Texture};
use crate::vec3::{Color, Point3};

pub fn build(rng: &mut ThreadRng) -> ShapeList {
    let mut shapes = ShapeList::default();

    let perlin: Rc<dyn Texture> = Rc::new(Perlin::new(4.0, 7, rng));
    let perlin: Rc<dyn Material> = Rc::new(Lambertian::new(perlin));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&perlin),
    )));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin,
    )));

    let diffuse_light: Rc<dyn Material> =
        Rc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    shapes.add(Rc::new(XyRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Rc::clone(&diffuse_light),
    )));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light,
    )));

    shapes
}
