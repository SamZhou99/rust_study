extern crate image;
extern crate num_complex;
use chrono::Local;

pub fn demo() -> String {
    let imgx = 256;
    let imgy = 256;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (1.0 * x as f32) as u8;
        let b = (2.0 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(y, x);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    let now = Local::now();
    let file_name = format!("{}.png", now.format("%Y%m%d_%H%M%S"));
    imgbuf.save(&file_name).unwrap();
    return file_name;
}
