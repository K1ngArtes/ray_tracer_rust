use core::ops;
use std::ops::Neg;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    // Pixels are written from left to right, top to bottom
    let mut row = image_height - 1;
    while row >= 0 {
        for col in 0..image_height {
            eprintln!("Scanlines remaining: {}", col);
            let r: f64 = row as f64 / (image_width - 1) as f64;
            let g: f64 = col as f64 / (image_height - 1) as f64;
            let b: f64 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
        row -= 1;
        eprintln!("Done");
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn add_vec3_test() {
    let v1 = Vec3{x:1.0, y:2.0, z:3.0};
    let v2 = Vec3{x:4.0, y:5.0, z:6.0};

    let v3 = v1 + v2;

    assert_eq!(Vec3{x:5.0, y:7.0, z:9.0}, v3);
}

#[test]
fn sub_vec3_test() {
    let v1 = Vec3{x:1.0, y:2.0, z:3.0};
    let v2 = Vec3{x:4.0, y:5.0, z:6.0};

    let v3 = v1 - v2;

    assert_eq!(Vec3{x:-3.0, y:-3.0, z:-3.0}, v3);
}

#[test]
fn neg_vec3_test() {
    let v1 = Vec3{x:1.0, y:2.0, z:3.0};

    let v3 = -v1;

    assert_eq!(Vec3{x:-1.0, y:-2.0, z:-3.0}, v3);
}
