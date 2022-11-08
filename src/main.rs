use crate::hitable::Hitable;
use rand::random;
use std::fmt::{format, Debug};
use std::sync::Arc;
use std::time;

#[macro_use]
extern crate log;

mod camera;
mod hitable;
mod hitable_list;
mod materials;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    let start_time = time::Instant::now();
    env_logger::init();
    let nx = 800;
    let ny = 400;
    let ns = 100;
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);
    let list: Vec<Box<dyn Hitable>> = vec![
        Box::new(sphere::new(
            vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(materials::new_lambertian(vec3::new(0.1, 0.2, 0.5))),
        )),
        Box::new(sphere::new(
            vec3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(materials::new_lambertian(vec3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(sphere::new(
            vec3::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(materials::new_metal(vec3::new(0.8, 0.6, 0.2), 0.4)),
        )),
        Box::new(sphere::new(
            vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(materials::new_dielectric(1.5)),
        )),
        Box::new(sphere::new(
            vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Arc::new(materials::new_dielectric(1.5)),
        )), // Box::new(sphere::new(
            //     vec3::new(-1.0, 0.0, -1.0),
            //     0.5,
            //     Arc::new(materials::new_metal(vec3::new(0.8, 0.8, 0.8), 1.0)),
            // )),
    ];

    // let radius = f64::cos(std::f64::consts::PI / 4.0);
    // let list2: Vec<Box<dyn Hitable>> = vec![
    //     Box::new(sphere::new(
    //         vec3::new(-radius, 0.0, -1.0),
    //         radius,
    //         Arc::new(materials::new_lambertian(vec3::new(0.0, 0.0, 1.0))),
    //     )),
    //     Box::new(sphere::new(
    //         vec3::new(radius, 0.0, -1.0),
    //         radius,
    //         Arc::new(materials::new_lambertian(vec3::new(1.0, 0.0, 0.0))),
    //     )),
    // ];

    let world: Box<dyn Hitable> = Box::new(hitable_list::new(list));
    let look_from = vec3::new(-2.0, 2.0, 1.0);
    let look_at = vec3::new(0.0, 0.0, -1.0);
    let vup = vec3::new(0.0, 1.0, 0.0);
    let cam = camera::new(look_from, look_at, vup, 90.0, nx as f64 / ny as f64);

    let image_name = format!(
        "raytracer_{}.png",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    info!("Generating image `{}` ...", image_name);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f64 + random::<f64>()) / nx as f64;
            let v = ((ny - y) as f64 + random::<f64>()) / ny as f64;
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
    info!(
        "Done generating image in {} seconds",
        start_time.elapsed().as_secs()
    );

    imgbuf.save(image_name).unwrap();
}
