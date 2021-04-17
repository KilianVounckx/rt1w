use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Rc;

use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use crate::shape::{moving_sphere::MovingSphere, shape_list::ShapeList, sphere::Sphere};
use crate::texture::checkers::Checkers;
use crate::vec3::{Color, Point3, Vec3};

pub fn build(rng: &mut ThreadRng) -> ShapeList {
    let mut world = ShapeList::default();

    let ground: Rc<dyn Material> = Rc::new(Lambertian::new(Rc::new(Checkers::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let choose_mat = rng.gen::<f64>();

                if choose_mat < 0.8 {
                    let color = Color::rand(rng) * Color::rand(rng);
                    let material: Rc<dyn Material> = Rc::new(Lambertian::from_color(color));
                    let center2 = center + Vec3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center, center2, 0.0, 1.0, 0.2, material,
                    )));
                } else if choose_mat < 0.95 {
                    let color = Color::rand_between(0.5, 1.0, rng);
                    let fuzz = 0.5 * rng.gen::<f64>();
                    let material: Rc<dyn Material> = Rc::new(Metal::new(color, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.4, 0.2, 1.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}
