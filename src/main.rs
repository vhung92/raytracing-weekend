use crate::hitable::{Hitable};
use crate::vec3::Vec3;
use crate::ray::Ray;

#[macro_use] extern crate log;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = vec3::dot(r.direction(), r.direction());
    let b = 2.0 * vec3::dot(oc, r.direction());
    let c = vec3::dot(oc, oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - f64::sqrt(discriminant)) / (2.0 * a)
    }
}

fn color (r: Ray, world: &Box<dyn Hitable>) -> Vec3 {
    return match world.hit(r, 0.0, f64::MAX) {
        Some(rec) => {
            vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) * 0.5
        }
        _ => {
            let unit_direction = vec3::unit_vec(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * vec3::one() + t * vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    env_logger::init();
    let nx = 800;
    let ny = 400;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let lower_left_corner = vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::new(0.0, 0.0, 0.0);

    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(sphere::new(vec3::new(0.0,0.0,-1.0), 0.5)));
    list.push(Box::new(sphere::new(vec3::new(0.0,-100.5,-1.0), 100.0)));

    let world: Box<dyn Hitable> = Box::new(hitable_list::new(list));

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f64 / nx as f64;
        let v = (ny-y) as f64 / ny as f64;
        let r = ray::new(origin, lower_left_corner + horizontal*u + vertical*v);

        let p = r.point_at_parameter(2.0);
        let col = color(r, &world);
        let ir = (col.r() * 255.99) as u8;
        let ig = (col.g() * 255.99) as u8;
        let ib = (col.b() * 255.99) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
        debug!("x: {}, y: {}, color: {}", x, y, col);
    }

    imgbuf.save("blah.png").unwrap();
}
