use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{Color, Vec3};

#[derive(Clone, Copy)]
pub enum MaterialEnum {
    Lambertian { albedo: Color },
    Metal {
        albedo: Color,
        fuzziness: f64
    },
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
        r_in: &Ray,
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
            MaterialEnum::Metal { albedo: albedoVal, fuzziness: f } => {
                assert!(*f >= 0.0 && *f <= 1.0);
                let reflected = Vec3::reflect(r_in.dir.unit_vector(), hit_record.normal);
                *scattered = Ray::new(hit_record.p, reflected + *f * Vec3::random_in_unit_sphere());
                *attenuation = *albedoVal;

                return scattered.dir.dot(hit_record.normal) > 0.0;
            }
        }
    }
}
