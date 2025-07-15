fn main() {
    let file_name = test_image::image_demo::demo();
    println!("save img : {}", &file_name);
    test_image::image_filter::gray(&file_name, "./gray.jpg").unwrap();
    test_image::image_filter::blur(&file_name, "./blur.jpg").unwrap();
    test_image::image_filter::adjust_contrast(&file_name, "./adjust_contrast.jpg").unwrap();
    test_image::image_filter::brighten(&file_name, "./brighten.jpg").unwrap();
    test_image::image_filter::crop(&file_name, "./crop.jpg").unwrap();
    test_image::image_filter::rotate(&file_name, "./rotate.jpg").unwrap();
}
