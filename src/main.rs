mod image_info;
mod image_transformation;
mod utils;

use std::path::Path;

use crate::image_info::{average_brightness, calc_blur};
use crate::image_transformation::get_negative;
use image::{DynamicImage, GenericImageView};

fn main() {
    let dir: &str = "images/pulp_fiction.png";
    let img: DynamicImage = image::open(dir).unwrap();
    // let (width, height) = img.dimensions();
    // let pix = img.get_pixel(0, 0);
    println!("dimensions: {:?}", img.dimensions());

    let brightness: f64 = average_brightness(img.to_rgb8().to_vec());
    println!("brightness: {:?}", brightness);

    let blur_score = calc_blur(&img);
    println!("blur_score: {:?}", blur_score);

    let negative_img = get_negative(&img);
    let path = Path::new("images/negative_image.png");
    negative_img.save(path).unwrap();
    println!("negative image saved at: {:?}", path);
}
