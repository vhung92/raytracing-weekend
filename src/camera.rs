use crate::ray;
use crate::vec3;

pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

pub fn new(
    look_from: vec3::Vec3,
    look_at: vec3::Vec3,
    vup: vec3::Vec3,
    vertical_fov: f64,
    aspect: f64,
) -> Camera {
    let theta = vertical_fov * std::f64::consts::PI / 180.0;
    let half_height = f64::tan(theta / 2.0);
    let half_width = aspect * half_height;
    let origin = look_from;

    let w = vec3::unit_vec(look_from - look_at);
    let u = vec3::unit_vec(vec3::cross(vup, w));
    let v = vec3::cross(w, u);
    let a = half_width * u;
    let b = half_height * v;
    let lower_left_corner = origin - a - b - w;

    Camera {
        lower_left_corner,
        horizontal: 2.0 * half_width * u,
        vertical: 2.0 * half_height * v,
        origin,
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v,
        )
    }
}
