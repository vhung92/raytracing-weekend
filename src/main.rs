use crate::hitable::Hitable;
use rand::{random, Rng, thread_rng};
use std::sync::Arc;
use std::time;
use crate::materials::Material;
use crate::utils::color;

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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);
    // let list: Vec<Box<dyn Hitable>> = vec![
    //     Box::new(sphere::new(
    //         vec3::new(0.0, 0.0, -1.0),
    //         0.5,
    //         Arc::new(materials::new_lambertian(vec3::new(0.1, 0.2, 0.5))),
    //     )),
    //     Box::new(sphere::new(
    //         vec3::new(0.0, -100.5, -1.0),
    //         100.0,
    //         Arc::new(materials::new_lambertian(vec3::new(0.8, 0.8, 0.0))),
    //     )),
    //     Box::new(sphere::new(
    //         vec3::new(1.0, 0.0, -1.0),
    //         0.5,
    //         Arc::new(materials::new_metal(vec3::new(0.8, 0.6, 0.2), 0.4)),
    //     )),
    //     Box::new(sphere::new(
    //         vec3::new(-1.0, 0.0, -1.0),
    //         0.5,
    //         Arc::new(materials::new_dielectric(1.5)),
    //     )),
    //     Box::new(sphere::new(
    //         vec3::new(-1.0, 0.0, -1.0),
    //         -0.45,
    //         Arc::new(materials::new_dielectric(1.5)),
    //     )),
    // ];

    let world = random_scene();
    let look_from = vec3::new(13.0, 2.0, 3.0);
    let look_at = vec3::new(0.0, 0.0, 0.0);
    let vup = vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = camera::new(look_from, look_at, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

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
        for _ in 0..samples_per_pixel {
            let u = (x as f64 + random::<f64>()) / image_width as f64;
            let v = ((image_height - y) as f64 + random::<f64>()) / image_height as f64;
            let ray = cam.get_ray(u, v);
            // let p = ray.point_at_parameter(2.0);
            col += utils::color(ray, &world, 0);
        }
        col /= samples_per_pixel as f64;
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

fn random_scene() -> Box<dyn Hitable> {
    let mut list: Vec<Box<dyn Hitable>> = Vec::new();

    let ground_material = materials::new_lambertian(vec3::new(0.5, 0.5, 0.5));

    list.push(Box::new(sphere::new(
        vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Arc::new(ground_material),
    )));

    let foo = vec3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = vec3::new(a as f64 + 0.9*random::<f64>(), 0.2, b as f64 + 0.9*random::<f64>());

            if (center - foo).length() > 0.9 {
                let mut sphere_material: Arc<dyn Material>;
                sphere_material = Arc::new(materials::new_dielectric(1.5));
                if choose_mat < 0.8 {
                    let albedo = vec3::random_vec() * vec3::random_vec();
                    sphere_material = Arc::new(materials::new_lambertian(albedo));
                }
                else if choose_mat < 0.95 {
                    let albedo = vec3::random_vec_bounded(0.5, 1.0);
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    sphere_material = Arc::new(materials::new_metal(albedo, fuzz));
                }
                list.push(Box::new(sphere::new(
                    center, 0.2, sphere_material
                )))
            }
        }
    }

    let material1 = materials::new_dielectric(1.5);
    list.push(Box::new(sphere::new(
        vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(material1)
    )));

    let material2 = materials::new_lambertian(vec3::new(0.4, 0.2, 0.1));
    list.push(Box::new(sphere::new(
        vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(material2)
    )));

    let material3 = materials::new_metal(vec3::new(0.7, 0.6, 0.5), 0.0);
    list.push(Box::new(sphere::new(
        vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(material3)
    )));

    Box::new(hitable_list::new(list))
}