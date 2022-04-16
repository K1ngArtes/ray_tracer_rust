pub static INFINITY: f64 = f64::INFINITY;
pub static PI: f64 = std::f64::consts::PI;

fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}
