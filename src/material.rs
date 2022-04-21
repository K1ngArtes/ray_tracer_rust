use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::vector::{Color, Point3, Vec3};
use dyn_clone::DynClone;

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait Material: DynClone {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Default, Clone)]
pub struct DiffuseMaterial {}

impl Material for DiffuseMaterial {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool {
        todo!()
    }
}