use rand;
use rand::rngs::ThreadRng;

use rtc::camera::{Camera, CameraConfig};
use rtc::ray::Ray;
use rtc::scenes::{
    cornell_box, cornell_smoke, earth, last, random, simple_light, two_perlin_spheres, two_spheres,
};
use rtc::shape::Shape;
use rtc::vec3::{Color, Point3};

fn ray_color<T: Shape>(
    ray: &Ray,
    background: Option<Color>,
    world: &T,
    depth: u32,
    rng: &mut ThreadRng,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY, rng) {
        let emitted = rec.material().emitted(rec.u(), rec.v(), rec.point());
        if let Some((scattered, attenuation)) = rec.material().scatter(&ray, &rec, rng) {
            return attenuation * ray_color(&scattered, background, world, depth - 1, rng);
        }
        return emitted;
    }

    if let Some(background) = background {
        return background;
    }

    let unit = ray.direction().normalized();
    let t = 0.5 * (unit.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_width: u32 = 400;
    let mut samples_per_pixel: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let mut rng = rand::thread_rng();
    let world;
    let from;
    let mut at = Point3::new(0.0, 0.0, 0.0);
    let vfov;
    let mut background = None;
    let mut aperture = 0.0;

    let case = 0;
    match case {
        1 => {
            world = random::build(&mut rng);
            from = Point3::new(13.0, 2.0, 3.0);
            vfov = 20.0;
            aperture = 1.0;
        }
        2 => {
            world = two_spheres::build();
            from = Point3::new(13.0, 2.0, 3.0);
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres::build(&mut rng);
            from = Point3::new(13.0, 2.0, 3.0);
            vfov = 20.0;
        }
        4 => {
            world = earth::build();
            from = Point3::new(13.0, 2.0, 3.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_light::build(&mut rng);
            background = Some(Color::new(0.0, 0.0, 0.0));
            samples_per_pixel = 400;
            from = Point3::new(26.0, 3.0, 6.0);
            at = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box::build();
            background = Some(Color::new(0.0, 0.0, 0.0));
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            from = Point3::new(278.0, 278.0, -800.0);
            at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke::build();
            background = Some(Color::new(0.0, 0.0, 0.0));
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            from = Point3::new(278.0, 278.0, -800.0);
            at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = last::build(&mut rng);
            background = Some(Color::new(0.0, 0.0, 0.0));
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 200;
            from = Point3::new(478.0, 278.0, -600.0);
            at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let camera = Camera::new(CameraConfig {
        from,
        at,
        vfov,
        aperture,
        focus: 10.0,
        aspect_ratio,
        ..CameraConfig::default()
    });

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = i as f64 / (image_width - 1) as f64;
                let v = j as f64 / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v, &mut rng);
                color += ray_color(&ray, background, &world, MAX_DEPTH, &mut rng);
            }
            println!("{}", Color::format_color(color, samples_per_pixel));
        }
    }
    eprintln!("\nDone.\n");
}
