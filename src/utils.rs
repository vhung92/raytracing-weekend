use crate::hitable::Hitable;
use crate::{ray, vec3};
use rand::random;
use std::borrow::Borrow;

pub(crate) fn reflect(v: vec3::Vec3, n: vec3::Vec3) -> vec3::Vec3 {
    v - 2.0 * vec3::dot(v, n) * n
}

pub(crate) fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p = vec3::one();
    while p.squared_length() >= 1.0 {
        p = 2.0 * vec3::new(random::<f64>(), random::<f64>(), random::<f64>())
            - vec3::new(1.0, 1.0, 1.0);
    }
    p
}

pub(crate) fn color(r: ray::Ray, world: &Box<dyn Hitable>, depth: i32) -> vec3::Vec3 {
    return match world.hit(r, 0.001, f64::MAX) {
        Some(rec) => {
            let scattered = rec.material.scatter(r, rec.borrow());
            if depth < 50 {
                if let Some((attn, sc)) = scattered {
                    return attn * color(sc, world, depth + 1);
                }
            }
            return vec3::new(0.0, 0.0, 0.0);
        }
        _ => {
            let unit_direction = vec3::unit_vec(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * vec3::one() + t * vec3::new(0.5, 0.7, 1.0)
        }
    };
}

pub(crate) fn refract(v: vec3::Vec3, n: vec3::Vec3, ni_over_nt: f64) -> Option<vec3::Vec3> {
    let uv = vec3::unit_vec(v);
    let dt = vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * f64::sqrt(discriminant);
        return Option::Some(refracted);
    }
    None
}

pub(crate) fn schlick_approximation(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}
