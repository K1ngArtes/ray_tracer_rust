use crate::hittable::{HitRecord, Hittable, Sphere};
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

pub fn ray_color(ray: Ray) -> Color {
    let sphere = Sphere {
        radius: 0.5,
        center: Point3::new(0.0, 0.0, -1.0),
    };

    let mut hit_record: HitRecord = HitRecord::default();
    if sphere.hit(&ray, 0.000001, 1000.0, &mut hit_record) {
        let hit_normal = hit_record.normal;
        return 0.5 * Color::new(hit_normal.x + 1.0, hit_normal.y + 1.0, hit_normal.z + 1.0);
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    // Interpolate from white to blue
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
