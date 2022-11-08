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
    aspect_ratio: f64,
) -> Camera {
    let theta = f64::to_radians(vertical_fov);
    let h = f64::tan(theta / 2.0);
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = vec3::unit_vec(look_from - look_at);
    let u = vec3::unit_vec(vec3::cross(vup, w));
    let v = vec3::cross(w, u);

    let origin = look_from;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
