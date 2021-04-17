use rand::rngs::ThreadRng;

use std::fmt::{self, Display};
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::{rand, rand_between};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn rand(rng: &mut ThreadRng) -> Self {
        Self {
            x: rand(rng),
            y: rand(rng),
            z: rand(rng),
        }
    }

    pub fn rand_between(min: f64, max: f64, rng: &mut ThreadRng) -> Self {
        Self {
            x: rand_between(min, max, rng),
            y: rand_between(min, max, rng),
            z: rand_between(min, max, rng),
        }
    }

    pub fn rand_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::rand_between(-1.0, 1.0, rng);
            if p.norm_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn rand_in_unit_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self {
                x: rand_between(-1.0, 1.0, rng),
                y: rand_between(-1.0, 1.0, rng),
                z: 0.0,
            };
            if p.norm_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn rand_unit(rng: &mut ThreadRng) -> Self {
        Self::rand_in_unit_sphere(rng).normalized()
    }

    pub fn rand_in_hemisphere(normal: Vec3, rng: &mut ThreadRng) -> Self {
        let in_unit = Self::rand_in_unit_sphere(rng);
        if Self::dot(in_unit, normal) > 0.0 {
            in_unit
        } else {
            -in_unit
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn norm(self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    pub fn norm_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(self) -> Self {
        self / self.norm()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(self, n: Self) -> Self {
        self - n * Self::dot(self, n) * 2.0
    }

    pub fn refract(self, n: Self, eta: f64) -> Self {
        let cos_th = f64::min(Self::dot(-self, n), 1.0);
        let perp = (self + n * cos_th) * eta;
        let par = n * -f64::sqrt(f64::abs(1.0 - perp.norm_squared()));
        perp + par
    }

    pub fn near_zero(self) -> bool {
        const S: f64 = 1e-8;
        f64::abs(self.x) < S && f64::abs(self.y) < S && f64::abs(self.z) < S
    }

    pub fn format_color(self, samples_per_pixel: u32) -> String {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        let scale = 1.0 / samples_per_pixel as f64;
        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);

        format!(
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32,
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bounds: got {}", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds: got {}", index),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        };
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        };
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Self::Output) -> Self::Output {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Sum<Self> for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::default(), |a, b| a + b)
    }
}
