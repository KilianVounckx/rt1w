use rand::rngs::ThreadRng;

use super::Texture;
use crate::perlin::Noise;
use crate::vec3::{Color, Point3};

pub struct Perlin {
    noise: Noise,
    scale: f64,
    depth: i32,
}

impl Perlin {
    pub fn new(scale: f64, depth: i32, rng: &mut ThreadRng) -> Self {
        Self {
            noise: Noise::new(rng),
            scale,
            depth,
        }
    }
}

impl Texture for Perlin {
    fn color(&self, _: f64, _: f64, point: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + f64::sin(
                    self.scale * point.z() + 10.0 * self.noise.turbulence(point, self.depth),
                ))
    }
}
