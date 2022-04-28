use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::{Color, Vec3};

#[derive(Clone, Copy)]
pub enum MaterialEnum {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzziness: f64 },
    Dielectric { index_of_refraction: f64 },
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
            MaterialEnum::Lambertian { albedo: albedo_val } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                }

                *scattered = Ray::new(hit_record.p, scatter_direction);
                *attenuation = *albedo_val;
                true
            }
            MaterialEnum::Metal {
                albedo: albedo_val,
                fuzziness: f,
            } => {
                let reflected = Vec3::reflect(r_in.dir.unit_vector(), hit_record.normal);
                *scattered = Ray::new(hit_record.p, reflected + *f * Vec3::random_in_unit_sphere());
                *attenuation = *albedo_val;

                return scattered.dir.dot(hit_record.normal) > 0.0;
            }
            MaterialEnum::Dielectric {
                index_of_refraction: index_of_ref,
            } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio;
                if hit_record.is_front_face {
                    refraction_ratio = 1.0 / index_of_ref;
                } else {
                    refraction_ratio = *index_of_ref;
                }

                let unit_direction = r_in.dir.unit_vector();
                let refracted = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);

                *scattered = Ray::new(hit_record.p, refracted);
                return true;
            }
        }
    }
}
