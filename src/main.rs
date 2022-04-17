mod hittable;
mod ray;
mod util;
mod vector;

use crate::hittable::{HittableList, Sphere};
use ray::Ray;
use vector::{Color, Point3, Vec3};

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

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
            let u = (col as f64) / (image_width as f64 - 1.0);
            let v = (row as f64) / (image_height as f64 - 1.0);
            let r = camera.ray(u, v);
            let pixel_color = ray::ray_color(r, &world);

            write_color(pixel_color);
        }
        row -= 1;
        // eprintln!("Done");
    }
}

fn write_color(pixel_color: Color) {
    let ir: i32 = (255.999 * pixel_color.x) as i32;
    let ig: i32 = (255.999 * pixel_color.y) as i32;
    let ib: i32 = (255.999 * pixel_color.z) as i32;

    println!("{} {} {}", ir, ig, ib);
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

    let mut world = HittableList{objects: Vec::new()};
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
                point3_values[2].parse().unwrap()
            );
            let sphere = Sphere {
                radius,
                center,
            };
            world.objects.push(Box::new(sphere));
        }
        is_radius = !is_radius;
    }

    Ok(world)
}
