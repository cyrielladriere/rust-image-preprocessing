use crate::utils::{apply_noise_to_channel, combine_channels};
use image::{DynamicImage, GenericImageView, Rgb, RgbImage, RgbaImage};
use rand::seq::SliceRandom;
use rand::Rng;

pub fn get_negative(img: &DynamicImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut out_img = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut pix = img.get_pixel(x, y);
            for i in 0..3 {
                pix[i] = 255 as u8 - pix[i];
            }
            out_img.put_pixel(x, y, pix);
        }
    }
    out_img
}

pub fn get_gamma_log_transform(img: &DynamicImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut out_img = RgbaImage::new(width, height);
    let c = 255.0 / f64::log(255.0 + 1.0, 10.0);
    for y in 0..height {
        for x in 0..width {
            let mut pix = img.get_pixel(x, y);
            for i in 0..3 {
                pix[i] = (c * f64::log(pix[i] as f64 + 1.0, 10.0)) as u8;
            }
            out_img.put_pixel(x, y, pix);
        }
    }
    out_img
}

pub fn get_gamma_powlaw_transform(img: &DynamicImage, gamma: f64) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut out_img = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut pix = img.get_pixel(x, y);
            for i in 0..3 {
                pix[i] = (255.0 * f64::powf(pix[i] as f64 / 255.0, 1.0 / gamma)) as u8;
            }
            out_img.put_pixel(x, y, pix);
        }
    }
    out_img
}

pub fn get_gaussian_noise(img: &DynamicImage, std_dev: f64) -> RgbImage {
    // Convert the image to RGB
    let img_rgb = img.to_rgb8();

    // Generate noise with the same shape as that of the image
    let (width, height) = img_rgb.dimensions();
    let mut rng = rand::thread_rng();

    // Apply noise to each channel
    let r_channel = apply_noise_to_channel(&img_rgb, &mut rng, 0.0, std_dev, |pixel| pixel[0]);
    let g_channel = apply_noise_to_channel(&img_rgb, &mut rng, 0.0, std_dev, |pixel| pixel[1]);
    let b_channel = apply_noise_to_channel(&img_rgb, &mut rng, 0.0, std_dev, |pixel| pixel[2]);

    // Combine channels into a single image
    let img_noised = combine_channels(width, height, r_channel, g_channel, b_channel);

    img_noised
}

pub fn get_salt_and_pepper_noise(img: &DynamicImage, noise_percentage: f64) -> RgbImage {
    // Convert the image to RGB
    let img_rgb = img.to_rgb8();

    // Get the image size
    let (width, height) = img_rgb.dimensions();
    let img_size = (width * height) as usize;

    // Calculate the number of noisy pixels
    let noise_size = (noise_percentage * img_size as f64).round() as usize;

    // Generate random indices for the noise
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..img_size).collect();
    indices.shuffle(&mut rng);
    let random_indices = &indices[..noise_size];

    // Create the noised image
    let mut img_noised = img_rgb.clone();

    for &index in random_indices {
        let x = (index as u32) % width;
        let y = (index as u32) / width;
        let noise_value = if rng.gen::<bool>() { 255 } else { 0 };
        img_noised.put_pixel(x, y, Rgb([noise_value, noise_value, noise_value]));
    }

    img_noised
}
