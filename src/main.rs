fn main() {
    let nx = 800;
    let ny = 600;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = x as f64 / nx as f64;
        let g = y as f64 / ny as f64;
        let b = 0.2;

        let ir = (r * 255.99) as u8;
        let ig = (g * 255.99) as u8;
        let ib = (b * 255.99) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    imgbuf.save("blah.png").unwrap();
}
