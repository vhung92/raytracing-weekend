use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

pub fn new(center: Vec3, radius: f64) -> Sphere {
    Sphere{ center, radius }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - f64::sqrt(b*b-a*c)) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let hit_rec = HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                };
                return Some(hit_rec);
            }
            temp = (-b + f64::sqrt(b*b-a*c)) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let hit_rec = HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                };
                return Some(hit_rec);
            }
        }
        None
    }
}