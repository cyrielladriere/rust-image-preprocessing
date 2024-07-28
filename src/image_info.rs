use image::{DynamicImage, GrayImage};

use crate::utils::{apply_laplacian, calculate_variance};

// Calculate the average brightness of an image,
// returned as a float between 0 and 1.
pub fn average_brightness(raw_rgb: Vec<u8>) -> f64 {
    let mut sum = 0.0;
    for (i, _) in raw_rgb.iter().enumerate().step_by(3) {
        let r = raw_rgb[i] as f64;
        let g = raw_rgb[i + 1] as f64;
        let b = raw_rgb[i + 2] as f64;
        let pixel_brightness = (r / 255.0 + g / 255.0 + b / 255.0) / 3.0;
        sum += pixel_brightness;
    }
    sum / (raw_rgb.len() as f64 / 3.0)
}

pub fn calc_blur(img: &DynamicImage) -> f64 {
    // Convert image to grayscale
    let gray_image: GrayImage = img.to_luma8();

    // Apply Laplacian filter for edge detection
    let laplacian_image = apply_laplacian(&gray_image);

    // Calculate variance of the Laplacian image
    let laplacian_variance = calculate_variance(&laplacian_image);

    // Check blur condition based on variance of Laplacian image
    let threshold = 100.0;
    let blur_text = if laplacian_variance < threshold {
        "Blurry"
    } else {
        "Not Blurry"
    };
    println!("The image is: {}", blur_text);

    laplacian_variance
}
