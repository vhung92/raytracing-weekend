use std::borrow::Borrow;
use rand::random;
use crate::hitable::Hitable;
use crate::{ray, vec3};

pub(crate) fn reflect(v: vec3::Vec3, n: vec3::Vec3) -> vec3::Vec3 {
    v - 2.0 *vec3::dot(v, n) * n
}

pub(crate) fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p = vec3::one();
    while p.squared_length() >= 1.0 {
        p = 2.0* vec3::new(random::<f64>(), random::<f64>(), random::<f64>()) - vec3::new(1.0,1.0, 1.0);
    }
    p
}

pub(crate) fn color (r: ray::Ray, world: &Box<dyn Hitable>, depth: i32) -> vec3::Vec3 {
    return match world.hit(r, 0.001, f64::MAX) {
        Some(rec) => {
            let scattered = rec.material.scatter(r, rec.borrow());
            return if depth < 50 && scattered.is_some() {
                let attn = scattered.unwrap().0;
                let sc = scattered.unwrap().1;
                attn * color(sc, world, depth + 1)
            } else {
                vec3::new(0.0,0.0,0.0)
            }
        }
        _ => {
            let unit_direction = vec3::unit_vec(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * vec3::one() + t * vec3::new(0.5, 0.7, 1.0)
        }
    }
}