use image_compare;
// use image_compare::Algorithm;

fn main() {
    let image_one = image::open("image1.png")
        .expect("Could not find test-image")
        .into_rgb8();
    let image_two = image::open("image2.png")
        .expect("Could not find test-image")
        .into_rgb8();
    let result = image_compare::rgb_hybrid_compare(&image_one, &image_two)
        .expect("Images had different dimensions");

    println!("{:?}", result);

    // let image_one = image::open("image1.png")
    //     .expect("Could not find test-image")
    //     .into_luma8();
    // let image_two = image::open("image2.png")
    //     .expect("Could not find test-image")
    //     .into_luma8();
    // let result =
    //     image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &image_one, &image_two)
    //         .expect("Images had different dimensions");

    // println!("{:?}", result);
}
