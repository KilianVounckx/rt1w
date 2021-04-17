use rand::rngs::ThreadRng;
use rand::Rng;

pub mod aabb;
pub mod camera;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod scenes;
pub mod shape;
pub mod texture;
pub mod vec3;

fn rand(rng: &mut ThreadRng) -> f64 {
    rng.gen::<f64>()
}

fn rand_between(min: f64, max: f64, rng: &mut ThreadRng) -> f64 {
    rng.gen_range(min..max)
}

fn rand_int(min: i32, max: i32, rng: &mut ThreadRng) -> i32 {
    rng.gen_range(min..=max)
}
