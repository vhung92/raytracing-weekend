use crate::hitable::HitRecord;
use crate::{ray, utils, vec3};
use rand::random;

pub trait Material {
    fn scatter(&self, r_in: ray::Ray, rec: &HitRecord) -> Option<(vec3::Vec3, ray::Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Lambertian {
    albedo: vec3::Vec3,
}

pub(crate) fn new_lambertian(albedo: vec3::Vec3) -> Lambertian {
    Lambertian { albedo }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: ray::Ray, rec: &HitRecord) -> Option<(vec3::Vec3, ray::Ray)> {
        let target = rec.p + rec.normal + utils::random_in_unit_sphere();
        Some((self.albedo, ray::new(rec.p, target - rec.p)))
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Metal {
    albedo: vec3::Vec3,
    fuzz: f64,
}

pub(crate) fn new_metal(albedo: vec3::Vec3, mut fuzz: f64) -> Metal {
    if fuzz > 1.0 {
        fuzz = 1.0;
    }
    Metal { albedo, fuzz }
}

impl Material for Metal {
    fn scatter(&self, r_in: ray::Ray, rec: &HitRecord) -> Option<(vec3::Vec3, ray::Ray)> {
        let reflected = utils::reflect(vec3::unit_vec(r_in.direction()), rec.normal);
        let scattered = ray::new(
            rec.p,
            reflected + self.fuzz * utils::random_in_unit_sphere(),
        );
        if vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Dielectric {
    ref_idx: f64,
}

pub(crate) fn new_dielectric(ref_idx: f64) -> Dielectric {
    Dielectric { ref_idx }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: ray::Ray, rec: &HitRecord) -> Option<(vec3::Vec3, ray::Ray)> {
        let mut outward_normal = rec.normal;
        let reflected = utils::reflect(r_in.direction(), rec.normal);
        let mut ni_over_nt = 1.0 / self.ref_idx;
        let attn = vec3::one();
        let mut cosine = -vec3::dot(r_in.direction(), rec.normal) / r_in.direction().length();
        let mut reflect_prob = 1.0;
        if vec3::dot(r_in.direction(), rec.normal) > 0.0 {
            outward_normal = -1.0 * outward_normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * vec3::dot(r_in.direction(), rec.normal) / r_in.direction().length();
        }

        let mut scattered = ray::new(rec.p, reflected);

        if let Some(refracted) = utils::refract(r_in.direction(), outward_normal, ni_over_nt) {
            reflect_prob = utils::schlick_approximation(cosine, self.ref_idx);
            if random::<f64>() < reflect_prob {
                return Some((attn, scattered));
            }
            scattered = ray::new(rec.p, refracted);
        }

        Some((attn, scattered))
    }
}
