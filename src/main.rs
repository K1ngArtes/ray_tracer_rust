mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vector;

use crate::hittable::{HittableList, Sphere};
use material::MaterialEnum;
use vector::{Color, Point3, Vec3};

use crate::ray::ray_color;
use camera::Camera;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

static SAMPLES_PER_PIXEL: i32 = 10;
static MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let world: HittableList = load_world_file().unwrap();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
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

    let duration = start.elapsed();
    eprintln!("Time elapsed is: {:?}", duration);
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

fn load_world_file() -> Result<HittableList, Error> {
    let path = "world.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut world = HittableList {
        objects: Vec::new(),
    };
    let mut radius = 0.0;
    let mut center = Vec3::default();
    let mut material_num: i32 = -1;
    let mut material: MaterialEnum;
    for (i, line) in buffered.lines().enumerate().map(|(i, l)| (i, l.unwrap())) {
        // Comment
        // Radius
        // Center
        // Material num
        // Material albedo color (or a special param)
        match i % 5 {
            0 => continue,
            1 => radius = parse_radius(&line),
            2 => {
                center = parse_center(&line);
            }
            3 => {
                material_num = parse_material_num(&line);
            }
            4 => {
                material = parse_material(&line, material_num);
                let sphere = Sphere {
                    radius,
                    center,
                    material,
                };
                world.objects.push(Box::new(sphere));
            }
            _ => {
                panic!("Should not get here")
            }
        }
    }

    Ok(world)
}

fn parse_material_num(line: &String) -> i32 {
    line.parse().unwrap()
}

fn parse_material(line: &String, material_num: i32) -> MaterialEnum {
    match material_num {
        1 => {
            return MaterialEnum::Lambertian {
                albedo: parse_color(line),
            }
        }
        2 => {
            let (albedo, fuzziness) = parse_color_with_fuzziness(line);
            return MaterialEnum::Metal { albedo, fuzziness };
        }
        3 => {
            let index_of_refraction = parse_index_of_refraction(line);
            return MaterialEnum::Dielectric {
                index_of_refraction,
            };
        }
        _ => {
            panic!("Should not get here")
        }
    }
}

fn parse_center(line: &String) -> Point3 {
    let point3_values: Vec<&str> = line.split(' ').to_owned().collect();
    return Point3::new(
        point3_values[0].parse().unwrap(),
        point3_values[1].parse().unwrap(),
        point3_values[2].parse().unwrap(),
    );
}

fn parse_color(line: &String) -> Color {
    let color_values: Vec<&str> = line.split(' ').to_owned().collect();
    return Color::new(
        color_values[0].parse().unwrap(),
        color_values[1].parse().unwrap(),
        color_values[2].parse().unwrap(),
    );
}

fn parse_color_with_fuzziness(line: &String) -> (Color, f64) {
    let color_values: Vec<&str> = line.split(' ').to_owned().collect();
    let fuzzy = color_values[3].parse().unwrap();
    assert!(fuzzy >= 0.0 && fuzzy <= 1.0);
    return (
        Color::new(
            color_values[0].parse().unwrap(),
            color_values[1].parse().unwrap(),
            color_values[2].parse().unwrap(),
        ),
        fuzzy,
    );
}

fn parse_radius(line: &String) -> f64 {
    return line.parse().unwrap();
}

fn parse_index_of_refraction(line: &String) -> f64 {
    return line.parse().unwrap();
}
