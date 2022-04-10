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
            let r: f64 = row as f64 / (image_width - 1) as f64;
            let g: f64 = col as f64 / (image_height - 1) as f64;
            let b: f64 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
        row -= 1;
    }
}
