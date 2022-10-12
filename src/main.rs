use std::sync::Arc;
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
mod materials;
mod utils;

fn main() {
    env_logger::init();
    let nx = 800;
    let ny = 400;
    let ns = 100;
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let mut list: Vec<Box<dyn Hitable>> = Vec::new();
    list.push(Box::new(sphere::new(vec3::new(0.0,0.0,-1.0), 0.5, Arc::new(materials::new_lambertian(vec3::new(0.8, 0.3, 0.3))))));
    list.push(Box::new(sphere::new(vec3::new(0.0,-100.5,-1.0), 100.0, Arc::new(materials::new_lambertian(vec3::new(0.8, 0.8, 0.0))))));
    list.push(Box::new(sphere::new(vec3::new(1.0,0.0,-1.0), 0.5, Arc::new(materials::new_metal(vec3::new(0.8, 0.6, 0.2))))));
    list.push(Box::new(sphere::new(vec3::new(-1.0,0.0,-1.0), 0.5, Arc::new(materials::new_metal(vec3::new(0.8, 0.8, 0.8))))));

    let world: Box<dyn Hitable> = Box::new(hitable_list::new(list));
    let cam = camera::new();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f64 + random::<f64>()) / nx as f64;
            let v = ((ny-y) as f64 + random::<f64>())  / ny as f64;
            let ray = cam.get_ray(u, v);
            // let p = ray.point_at_parameter(2.0);
            col += utils::color(ray, &world, 0);
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
