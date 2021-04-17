use crate::vec3::{Color, Point3};

pub mod checkers;
pub mod image;
pub mod perlin;
pub mod solid_color;

pub trait Texture {
    fn color(&self, u: f64, v: f64, point: Point3) -> Color;
}
