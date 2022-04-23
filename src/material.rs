use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{Color, Vec3};

#[derive(Clone)]
pub enum MaterialEnum {
    Lambertian { albedo: Color },
}

impl Default for MaterialEnum {
    fn default() -> Self {
        MaterialEnum::Lambertian {
            albedo: Color::default(),
        }
    }
}

impl MaterialEnum {
    pub fn scatter(
        &self,
        _r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialEnum::Lambertian { albedo: albedoVal } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                }

                *scattered = Ray::new(hit_record.p, scatter_direction);
                *attenuation = *albedoVal;
                true
            }
        }
    }
}
