use std::rc::Rc;

use super::{solid_color::SolidColor, Texture};
use crate::vec3::{Color, Point3};

pub struct Checkers {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl Checkers {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            odd: Rc::new(SolidColor::new(odd)),
            even: Rc::new(SolidColor::new(even)),
        }
    }
}

impl Default for Checkers {
    fn default() -> Self {
        Self {
            odd: Rc::new(SolidColor::from_rgb(0.0, 0.0, 0.0)),
            even: Rc::new(SolidColor::from_rgb(1.0, 1.0, 1.0)),
        }
    }
}

impl Texture for Checkers {
    fn color(&self, u: f64, v: f64, point: Point3) -> Color {
        let sines =
            f64::sin(10.0 * point.x()) * f64::sin(10.0 * point.y()) * f64::sin(10.0 * point.z());
        if sines < 0.0 {
            self.odd.color(u, v, point)
        } else {
            self.even.color(u, v, point)
        }
    }
}
