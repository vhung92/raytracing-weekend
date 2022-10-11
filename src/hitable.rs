use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub(crate) t: f64,
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
