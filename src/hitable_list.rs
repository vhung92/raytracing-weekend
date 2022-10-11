use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
}

pub fn new(l: Vec<Box<dyn Hitable>>) -> HitableList {
    HitableList{list: l}
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: HitRecord;
        let mut result = None;
        let mut closest_so_far = t_max;
        let hit_anything = false;
        for obj in self.list.iter() {
            match obj.hit(r, t_min, closest_so_far) {
                Some(hit_rec) => {
                    record = hit_rec;
                    closest_so_far = record.t;
                    result = Some(record);
                }
                None => continue
            }
        }
        result
    }
}