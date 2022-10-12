use crate::ray;
use crate::vec3;

pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

pub fn new() -> Camera {
    Camera {
        lower_left_corner: vec3::new(-2.0, -1.0, -1.0),
        horizontal: vec3::new(4.0, 0.0, 0.0),
        vertical: vec3::new(0.0, 2.0, 0.0),
        origin: vec3::new(0.0, 0.0, 0.0),
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v)
    }
}