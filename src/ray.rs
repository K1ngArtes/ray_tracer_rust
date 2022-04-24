use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::MaterialEnum;
use crate::util;
use crate::vector::{Color, Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(ray: Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record: HitRecord = HitRecord::default();
    if world.hit(&ray, 0.001, util::INFINITY, &mut hit_record) {
        let mut scattered = Ray::new(Point3::default(), Vec3::default());
        let mut attenuation: Color = Color::default();
        if hit_record
            .material
            .scatter(&ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    // Interpolate from white to blue
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
