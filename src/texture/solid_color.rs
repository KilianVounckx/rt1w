use crate::vec3::{Color, Point3};

use super::Texture;

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        Self {
            color: Color::default(),
        }
    }
}

impl Texture for SolidColor {
    fn color(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color
    }
}
