use crate::vec3::Vec3;
use crate::ray::Ray;

#[macro_use] extern crate log;

mod vec3;
mod ray;

fn color (r: Ray) -> Vec3 {
    let unit_direction = vec3::unit_vec(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    vec3::one() * (1.0-t) + vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    env_logger::init();
    let nx = 800;
    let ny = 600;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let lower_left_corner = vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::new(0.0, 0.0, 0.0);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f64 / nx as f64;
        let v = (ny-y) as f64 / ny as f64;
        let r = ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
        let col = color(r);
        let ir = (col.r() * 255.99) as u8;
        let ig = (col.g() * 255.99) as u8;
        let ib = (col.b() * 255.99) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
        debug!("x: {}, y: {}, color: {}", x, y, col);
    }

    imgbuf.save("blah.png").unwrap();
}
