use std::sync::Arc;
use crate::materials;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub(crate) t: f64,
    pub(crate) p: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) material: Arc<dyn materials::Material>,
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
