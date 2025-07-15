extern crate image;
use image::{GenericImageView, ImageError, ImageReader};

// 灰度
pub fn gray(open_image_path: &str, save_image_path: &str) -> Result<(), ImageError> {
    let image_from_path = ImageReader::open(open_image_path)?.decode()?;
    let img = image_from_path.grayscale();
    img.save(save_image_path)?;
    Ok(())
}

// 高斯模糊
pub fn blur(open_image_path: &str, save_image_path: &str) -> Result<(), ImageError> {
    let image_from_path = ImageReader::open(open_image_path)?.decode()?;
    let img = image_from_path.blur(10.0);
    img.save(save_image_path)?;
    Ok(())
}

// 对比度
pub fn adjust_contrast(open_image_path: &str, save_image_path: &str) -> Result<(), ImageError> {
    let image_from_path = ImageReader::open(open_image_path)?.decode()?;
    let img = image_from_path.adjust_contrast(10.0);
    img.save(save_image_path)?;
    Ok(())
}

// 亮度
pub fn brighten(open_image_path: &str, save_image_path: &str) -> Result<(), ImageError> {
    let image_from_path = ImageReader::open(open_image_path)?.decode()?;
    let img = image_from_path.brighten(100);
    img.save(save_image_path)?;
    Ok(())
}

// 裁剪
pub fn crop(open_image_path: &str, save_image_path: &str) -> Result<(), ImageError> {
    let image_from_path = ImageReader::open(open_image_path)?.decode()?;
    let (width, height) = image_from_path.dimensions();
    let img = image_from_path.crop_imm(0, 0, width / 2 as u32, height / 2 as u32);
    img.save(save_image_path)?;
    Ok(())
}

// 旋转
pub fn rotate(open_image_path: &str, save_image_path: &str) -> Result<(), ImageError> {
    let image_from_path = ImageReader::open(open_image_path)?.decode()?;
    let img = image_from_path.rotate90();
    img.save(save_image_path)?;
    Ok(())
}
