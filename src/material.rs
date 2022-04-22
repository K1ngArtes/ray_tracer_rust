use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::vector::{Color, Point3, Vec3};

#[derive(Clone)]
pub enum MaterialEnum {
    Diffuse
}

impl Default for MaterialEnum {
    fn default() -> Self { MaterialEnum::Diffuse }
}

impl MaterialEnum {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &Color,
        scattered: &Ray,
    ) -> bool {
        todo!()
    }
}
