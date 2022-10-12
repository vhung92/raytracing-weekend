use rand::random;
use crate::hitable::{Hitable};
use crate::vec3::Vec3;
use crate::ray::Ray;

#[macro_use] extern crate log;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;

fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p = vec3::one();
    while p.squared_length() >= 1.0 {
        p = 2.0* vec3::new(random::<f64>(), random::<f64>(), random::<f64>()) - vec3::new(1.0,1.0, 1.0);
    }
    p
}

fn color (r: Ray, world: &Box<dyn Hitable>) -> Vec3 {
    return match world.hit(r, 0.001, f64::MAX) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            0.5 * color(ray::new(rec.p, target-rec.p), world)
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
    let ns = 100;
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(sphere::new(vec3::new(0.0,0.0,-1.0), 0.5)));
    list.push(Box::new(sphere::new(vec3::new(0.0,-100.5,-1.0), 100.0)));

    let world: Box<dyn Hitable> = Box::new(hitable_list::new(list));
    let cam = camera::new();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f64 + random::<f64>()) / nx as f64;
            let v = ((ny-y) as f64 + random::<f64>())  / ny as f64;
            let ray = cam.get_ray(u, v);
            // let p = ray.point_at_parameter(2.0);
            col += color(ray, &world);
        }
        col /= ns as f64;
        let ir = (f64::sqrt(col.r()) * 255.99) as u8;
        let ig = (f64::sqrt(col.g()) * 255.99) as u8;
        let ib = (f64::sqrt(col.b()) * 255.99) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
        debug!("x: {}, y: {}, color: {}", x, y, col);
    }

    imgbuf.save("blah.png").unwrap();
}
