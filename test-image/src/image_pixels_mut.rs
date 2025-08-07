use image::{GenericImageView, Rgb};
extern crate image;

const FILE_IMG: &str = "test.png";

pub fn get_pixels_mut() {
    let mut img: image::DynamicImage = image::open(FILE_IMG).unwrap();
    write_pixel_rgb(&mut img);
}

fn write_pixel_rgb(img: &mut image::DynamicImage) {
    let (width, height) = img.dimensions();
    let mut img_buf = img.to_rgb8();
    for y in 0..height {
        for x in 0..width {
            let pixel = img_buf.get_pixel_mut(x, y);
            // 白色
            if pixel.0[0] >= 255 && pixel.0[1] >= 255 && pixel.0[2] >= 255 {
                // 设置成黑色
                img_buf.put_pixel(x, y, Rgb([0, 0, 0]));
            } else if pixel.0[0] <= 0 && pixel.0[1] <= 0 && pixel.0[2] <= 0 {
                // 设置成白色
                img_buf.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }
    img_buf.save(FILE_IMG).unwrap();
}
