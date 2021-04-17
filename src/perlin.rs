use rand::rngs::ThreadRng;

use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

pub struct Noise {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Noise {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut rand_vec = [Vec3::default(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            rand_vec[i as usize] = Vec3::rand_between(-1.0, 1.0, rng);
        }

        let perm_x = generate_perm(rng);
        let perm_y = generate_perm(rng);
        let perm_z = generate_perm(rng);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[(self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255])
                        as usize];
                }
            }
        }

        trilinear_interpolation(&mut c, u, v, w)
    }

    pub fn turbulence(&self, point: Point3, depth: i32) -> f64 {
        let mut accumulation = 0.0;
        let mut tmp = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulation += weight * self.noise(tmp);
            weight *= 0.5;
            tmp *= 2.0;
        }

        accumulation.abs()
    }
}

fn generate_perm(rng: &mut ThreadRng) -> [i32; POINT_COUNT] {
    let mut p = [0; POINT_COUNT];
    for i in 0..POINT_COUNT {
        p[i] = i as i32;
    }
    permute(&mut p, rng);
    p
}

fn permute(p: &mut [i32; POINT_COUNT], rng: &mut ThreadRng) {
    for i in (0..p.len()).rev() {
        let target = crate::rand_int(0, i as i32, rng) as i32;
        let tmp = p[i];
        p[i] = target;
        p[target as usize] = tmp;
    }
}

fn trilinear_interpolation(c: &mut [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accumulation = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accumulation += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * Vec3::dot(c[i][j][k], weight);
            }
        }
    }
    accumulation
}
