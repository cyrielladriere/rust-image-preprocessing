mod image_info;
mod image_transformation;
mod utils;

use std::path::Path;

use crate::image_info::{average_brightness, calc_blur};
use crate::image_transformation::{
    get_gamma_log_transform, get_gamma_powlaw_transform, get_negative, get_salt_and_pepper_noise,
};
use image::{DynamicImage, GenericImageView};
use image_transformation::get_gaussian_noise;

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

    let gamma_log = get_gamma_log_transform(&img);
    let path = Path::new("images/gamma_log_transform.png");
    gamma_log.save(path).unwrap();
    println!("gamma log transform saved at: {:?}", path);
    let gamma_log_brightness: f64 = average_brightness(gamma_log.to_vec());
    println!(
        "brightness of gamma log transform: {:?}",
        gamma_log_brightness
    );

    let gamma_powlaw = get_gamma_powlaw_transform(&img, 0.7);
    let path = Path::new("images/gamma_powlaw_transform.png");
    gamma_powlaw.save(path).unwrap();
    println!("gamma powlaw transform saved at: {:?}", path);
    let gamma_powlaw_brightness: f64 = average_brightness(gamma_powlaw.to_vec());
    println!(
        "brightness of gamma powlaw transform: {:?}",
        gamma_powlaw_brightness
    );

    let gaussian_noise = get_gaussian_noise(&img, 100.0);
    let path = Path::new("images/gaussian_noise_image.png");
    gaussian_noise.save(path).unwrap();
    println!("gaussian noise image saved at: {:?}", path);

    let salt_and_pepper_noise = get_salt_and_pepper_noise(&img, 0.2);
    let path = Path::new("images/salt_and_pepper_noise_image.png");
    salt_and_pepper_noise.save(path).unwrap();
    println!("salt and pepper noise image saved at: {:?}", path);
}
