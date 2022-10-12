use crate::hitable::HitRecord;
use crate::{utils, ray, vec3};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: ray::Ray, rec: &HitRecord) -> Option<(vec3::Vec3, ray::Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Lambertian {
    albedo: vec3::Vec3,
}

pub(crate) fn new_lambertian(albedo: vec3::Vec3) -> Lambertian {
    Lambertian {
        albedo
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(vec3::Vec3, ray::Ray)> {
        let target = rec.p + rec.normal + utils::random_in_unit_sphere();
        Some((self.albedo, ray::new(rec.p, target-rec.p)))
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Metal {
    albedo: vec3::Vec3,
}

pub(crate) fn new_metal(albedo: vec3::Vec3) -> Metal {
    Metal {
        albedo
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = utils::reflect(vec3::unit_vec(r_in.direction()), rec.normal);
        let scattered = ray::new(rec.p, reflected);
        return if vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}