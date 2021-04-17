use rand::rngs::ThreadRng;
use std::rc::Rc;

use crate::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    Material,
};
use crate::shape::{
    bvh_node::BvhNode, constant_medium::ConstantMedium, cube::Cube, moving_sphere::MovingSphere,
    rotate_y::RotateY, shape_list::ShapeList, sphere::Sphere, translate::Translate,
    xz_rect::XzRect, Shape,
};
use crate::texture::{image::Image, perlin::Perlin, Texture};
use crate::vec3::{Color, Point3, Vec3};

pub fn build(rng: &mut ThreadRng) -> ShapeList {
    let mut boxes1 = ShapeList::default();
    let ground: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));
    const BOXES_PER_SIDE: u32 = 32;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f64 * w;

            let x1 = x0 + w;
            let y1 = crate::rand_between(1.0, 101.0, rng);
            let z1 = z0 + w;

            boxes1.add(Rc::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Rc::clone(&ground),
            )));
        }
    }

    let mut shapes = ShapeList::default();
    shapes.add(Rc::new(BvhNode::new(boxes1, 0.0, 1.0, rng)));

    let light: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    shapes.add(Rc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let material: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1)));
    shapes.add(Rc::new(MovingSphere::new(
        center1, center2, 0.0, 1.0, 50.0, material,
    )));

    shapes.add(Rc::new(Sphere::new(
        Point3::new(260.0, 250.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    shapes.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary: Rc<dyn Shape> = Rc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    shapes.add(Rc::clone(&boundary));
    shapes.add(Rc::new(ConstantMedium::from_color(
        Rc::clone(&boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary: Rc<dyn Shape> = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    shapes.add(Rc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth: Rc<dyn Texture> = Rc::new(Image::new("images/earthmap.jpg"));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Rc::new(Lambertian::new(earth)),
    )));

    let perlin: Rc<dyn Texture> = Rc::new(Perlin::new(0.1, 7, rng));
    shapes.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new(perlin)),
    )));

    let mut boxes2 = ShapeList::default();
    let white: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    const N: u32 = 1000;
    for _ in 0..N {
        boxes2.add(Rc::new(Sphere::new(
            Point3::rand_between(0.0, 165.0, rng),
            10.0,
            Rc::clone(&white),
        )));
    }

    shapes.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(BvhNode::new(boxes2, 0.0, 1.0, rng)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    shapes
}
