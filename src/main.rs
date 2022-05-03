mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vector;
mod world;

use crate::hittable::HittableList;
use vector::{Color, Point3, Vec3};

use crate::ray::ray_color;
use camera::Camera;
use std::env;
use std::ops::Div;
use std::time::Instant;

static SAMPLES_PER_PIXEL: i32 = 10;
static MAX_DEPTH: i32 = 50;

fn main() {
    let args: Vec<String> = env::args().collect();
    // World
    let world: HittableList;
    if args.len() >= 2 && &args[1] == "random" {
        world = world::random_scene();
    } else {
        world = world::load_world_file().unwrap();
    }

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        aspect_ratio,
        0.1,
        dist_to_focus,
    );

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let start = Instant::now();
    // Pixels are written from left to right, top to bottom
    let mut row = image_height - 1;
    while row >= 0 {
        // eprintln!("Scanlines remaining: {}", row);
        for col in 0..image_width {
            let mut pixel_color = Color::default();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (col as f64 + util::random_double()) / (image_width as f64 - 1.0);
                let v = (row as f64 + util::random_double()) / (image_height as f64 - 1.0);
                let new_ray = camera.ray(u, v);
                pixel_color = pixel_color + ray_color(new_ray, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
        row -= 1;
    }

    let duration = start.elapsed().div(60);
    eprintln!("Time elapsed is: {:.2?} minutes", duration);
}

fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut ir = pixel_color.x;
    let mut ig = pixel_color.y;
    let mut ib = pixel_color.z;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    ir = f64::sqrt(ir * scale);
    ig = f64::sqrt(ig * scale);
    ib = f64::sqrt(ib * scale);

    println!(
        "{} {} {}",
        (256.0 * util::clamp(ir, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(ig, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(ib, 0.0, 0.999)) as i32
    );
}
