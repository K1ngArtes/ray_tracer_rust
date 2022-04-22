use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::Color;

#[derive(Clone)]
pub enum MaterialEnum {
    Diffuse,
}

impl Default for MaterialEnum {
    fn default() -> Self {
        MaterialEnum::Diffuse
    }
}

impl MaterialEnum {
    pub fn scatter(
        &self,
        _r_in: &Ray,
        _hit_record: &HitRecord,
        _attenuation: &Color,
        _scattered: &Ray,
    ) -> bool {
        todo!()
    }
}
