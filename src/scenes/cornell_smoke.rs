use std::rc::Rc;

use crate::material::{diffuse_light::DiffuseLight, lambertian::Lambertian, Material};
use crate::shape::{
    constant_medium::ConstantMedium, cube::Cube, rotate_y::RotateY, shape_list::ShapeList,
    translate::Translate, xy_rect::XyRect, xz_rect::XzRect, yz_rect::YzRect, Shape,
};
use crate::vec3::{Color, Point3, Vec3};

pub fn build() -> ShapeList {
    let mut shapes = ShapeList::default();

    let red: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green: Rc<dyn Material> = Rc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    shapes.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    shapes.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    shapes.add(Rc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    shapes.add(Rc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Rc::clone(&white),
    )));
    shapes.add(Rc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::clone(&white),
    )));
    shapes.add(Rc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Rc::clone(&white),
    )));

    let mut box1: Rc<dyn Shape> = Rc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Rc::clone(&white),
    ));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    shapes.add(Rc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let mut box2: Rc<dyn Shape> = Rc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        Rc::clone(&white),
    ));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    shapes.add(Rc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    shapes
}
