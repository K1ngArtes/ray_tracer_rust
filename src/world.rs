use crate::hittable::{HittableList, Sphere};
use crate::material::MaterialEnum;
use crate::util::{random_double, random_double_rng};
use crate::vector::{Color, Point3, Vec3};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

pub fn load_world_file() -> Result<HittableList, io::Error> {
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

pub fn random_scene() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let ground_color = MaterialEnum::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };

    let ground_sphere = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_color,
    };

    world.objects.push(Box::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = random_double();
            let center = Point3::new(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let sphere_material = MaterialEnum::Lambertian {
                        albedo: Color::random() * Color::random(),
                    };
                    world.objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_limit(0.5, 1.0);
                    let fuzz = random_double_rng(0.0, 0.5);
                    let sphere_material = MaterialEnum::Metal {
                        albedo,
                        fuzziness: fuzz,
                    };
                    world.objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else {
                    // glass
                    let sphere_material = MaterialEnum::Dielectric {
                        index_of_refraction: 1.5,
                    };
                    world.objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                }
            }
        }

        let material1 = MaterialEnum::Dielectric {
            index_of_refraction: 1.5,
        };
        world.objects.push(Box::new(Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: material1,
        }));

        let material2 = MaterialEnum::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        };
        world.objects.push(Box::new(Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: material2,
        }));

        let material3 = MaterialEnum::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzziness: 0.0,
        };
        world.objects.push(Box::new(Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: material3,
        }));
    }

    return world;
}
