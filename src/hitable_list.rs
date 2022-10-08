use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
}

pub fn new(l: Vec<Box<dyn Hitable>>) -> HitableList {
    HitableList{list: l}
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.list.iter() {
            if obj.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}