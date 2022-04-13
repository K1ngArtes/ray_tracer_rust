mod vector;
use vector::{Color, Point3, Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    // Pixels are written from left to right, top to bottom
    let mut row = image_height - 1;
    while row >= 0 {
        eprintln!("Scanlines remaining: {}", row);
        for col in 0..image_width {
            let u = (col as f64) / (image_width as f64 - 1.0);
            let v = (row as f64) / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(r);

            write_color(pixel_color);
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

struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

fn ray_color(ray: Ray) -> Color {
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

#[derive(Default)]
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b - discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        hit_record.normal = (hit_record.p - self.center) / self.radius;

        true
    }
}
