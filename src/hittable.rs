use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
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
        let outward_normal_unit = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal_unit);

        true
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.is_front_face = ray.dir.dot(*outward_normal) < 0.0;
        self.normal = if self.is_front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hits_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hits_anything = true;
                *hit_record = temp_record.clone();
                closest_so_far = temp_record.t;
            }
        }

        return hits_anything;
    }
}
