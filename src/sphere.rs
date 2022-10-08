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
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - f64::sqrt(b*b-a*c)) / a;
            if temp < t_max && temp > t_min {
                hit_rec.t = temp;
                hit_rec.p = r.point_at_parameter(hit_rec.t);
                hit_rec.normal = (hit_rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + f64::sqrt(b*b-a*c)) / a;
            if temp < t_max && temp > t_min {
                hit_rec.t = temp;
                hit_rec.p = r.point_at_parameter(hit_rec.t);
                hit_rec.normal = (hit_rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}