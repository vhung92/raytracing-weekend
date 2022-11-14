use rand::{Rng, thread_rng};
use crate::ray;
use crate::vec3;

pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
    u: vec3::Vec3,
    v: vec3::Vec3,
    w: vec3::Vec3,
    lens_radius: f64,
}

pub fn new(
    look_from: vec3::Vec3,
    look_at: vec3::Vec3,
    vup: vec3::Vec3,
    vertical_fov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64
) -> Camera {
    let theta = f64::to_radians(vertical_fov);
    let h = f64::tan(theta / 2.0);
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = vec3::unit_vec(look_from - look_at);
    let u = vec3::unit_vec(vec3::cross(vup, w));
    let v = vec3::cross(w, u);

    let origin = look_from;
    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

    let lens_radius = aperture / 2.0;

    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
        u, v, w,
        lens_radius
    }
}

fn random_in_unit_disk() -> vec3::Vec3 {
    loop {
        let p = vec3::new(thread_rng().gen_range(-1.0..1.0),thread_rng().gen_range(-1.0..1.0), 0.0);
        if p.squared_length() >= 1.0 { continue; }
        return p;
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        ray::new(
            self.origin + offset,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
        )
    }
}
