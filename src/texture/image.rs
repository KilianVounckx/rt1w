use image::GenericImageView;

use super::Texture;
use crate::vec3::{Color, Point3};

pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        let img = image::open(filename);
        let img = match img {
            Ok(img) => img,
            _ => {
                return Self {
                    pixels: Vec::new(),
                    width: 0,
                    height: 0,
                }
            }
        };

        let (width, height) = img.dimensions();
        let (width, height) = (width as usize, height as usize);

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        let mut pixels = vec![Color::default(); width * height];
        for (i, pixel) in img.pixels().enumerate() {
            let color = &pixel.2 .0[0..3];
            pixels[i] = Color::new(
                color[0] as f64 * COLOR_SCALE,
                color[1] as f64 * COLOR_SCALE,
                color[2] as f64 * COLOR_SCALE,
            );
        }

        Self {
            pixels,
            width,
            height,
        }
    }
}

impl Texture for Image {
    fn color(&self, u: f64, v: f64, _: Point3) -> Color {
        if self.pixels.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        }

        if j >= self.height {
            j = self.height - 1;
        }

        self.pixels[j * self.width + i]
    }
}
