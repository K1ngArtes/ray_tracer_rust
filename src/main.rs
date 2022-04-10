use core::ops;

mod vector;
use vector::{Vec3, Color};

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

            write_color(Vec3{x: r, y: g, z: b});
        }
        row -= 1;
        eprintln!("Done");
    }
}

fn write_color(pixel_color: Color) {
    let ir: i32 = (255.999 * pixel_color.x) as i32;
    let ig: i32 = (255.999 * pixel_color.y) as i32;
    let ib: i32 = (255.999 * pixel_color.z) as i32;

    println!("{} {} {}", ir, ig, ib);
}