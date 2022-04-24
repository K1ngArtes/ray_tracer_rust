#![allow(dead_code)]
use rand::Rng;

pub static INFINITY: f64 = f64::INFINITY;
pub static PI: f64 = std::f64::consts::PI;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

pub fn random_double_rng(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }

    return x;
}
