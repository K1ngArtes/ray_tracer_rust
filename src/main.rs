mod hittable;
mod material;
mod ray;
mod util;
mod vector;

use crate::hittable::{HittableList, Sphere};
use ray::Ray;
use vector::{Color, Point3, Vec3};
use material::MaterialEnum;

use crate::ray::ray_color;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

static SAMPLES_PER_PIXEL: i32 = 10;
static MAX_DEPTH: i32 = 50;

fn main() {
    let world: HittableList = load_world_file().unwrap();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

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
        // eprintln!("Done");
    }
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

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {
    fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            vertical,
            horizontal,
        }
    }

    fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

fn load_world_file() -> Result<HittableList, Error> {
    let path = "world.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut world = HittableList {
        objects: Vec::new(),
    };
    let mut is_radius = true;
    let mut radius = 0.0;
    for line in buffered.lines() {
        if is_radius {
            radius = line?.parse().unwrap();
        } else {
            let the_line = line?;
            let point3_values: Vec<&str> = the_line.split(' ').to_owned().collect();
            let center = Point3::new(
                point3_values[0].parse().unwrap(),
                point3_values[1].parse().unwrap(),
                point3_values[2].parse().unwrap(),
            );
            let sphere = Sphere { radius, center, material: MaterialEnum::Lambertian{ albedo: Color::default() } };
            world.objects.push(Box::new(sphere));
        }
        is_radius = !is_radius;
    }

    Ok(world)
}
