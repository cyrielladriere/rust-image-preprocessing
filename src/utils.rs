use image::{GrayImage, ImageBuffer, Rgb, RgbImage};
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};

pub fn calculate_variance(image: &Vec<f64>) -> f64 {
    let mean = mean(image);
    let variance = variance(image, mean);
    variance
}

fn mean(numbers: &Vec<f64>) -> f64 {
    if numbers.is_empty() {
        return -1.0;
    }
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len();

    sum / count as f64
}

fn variance(numbers: &Vec<f64>, mean_value: f64) -> f64 {
    if numbers.is_empty() {
        return -1.0;
    }
    let count = numbers.len();
    let sum_squared_diffs: f64 = numbers
        .iter()
        .map(|value| {
            let diff = value - mean_value;
            diff * diff
        })
        .sum();
    sum_squared_diffs / count as f64
}

pub fn apply_laplacian(image: &GrayImage) -> Vec<f64> {
    let kernel: [[i32; 3]; 3] = [[0, 1, 0], [1, -4, 1], [0, 1, 0]];

    let (width, height) = (image.width() as isize, image.height() as isize);
    let mut laplacian_image = vec![vec![0.0; width as usize]; height as usize];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut sum = 0;
            for ky in -1..=1 {
                for kx in -1..=1 {
                    let pixel_value = image.get_pixel((x + kx) as u32, (y + ky) as u32).0[0] as i32;
                    sum += pixel_value * kernel[(ky + 1) as usize][(kx + 1) as usize];
                }
            }
            laplacian_image[y as usize][x as usize] = sum as f64;
        }
    }

    laplacian_image
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .collect()
}

pub fn combine_channels(
    width: u32,
    height: u32,
    r_channel: Vec<u8>,
    g_channel: Vec<u8>,
    b_channel: Vec<u8>,
) -> RgbImage {
    let mut img_noised = ImageBuffer::new(width, height);

    for (x, y, pixel) in img_noised.enumerate_pixels_mut() {
        let idx = (y * width + x) as usize;
        *pixel = Rgb([r_channel[idx], g_channel[idx], b_channel[idx]]);
    }

    img_noised
}

pub fn apply_noise_to_channel<F>(
    img: &RgbImage,
    rng: &mut ThreadRng,
    mean: f64,
    std_dev: f64,
    get_channel: F,
) -> Vec<u8>
where
    F: Fn(&Rgb<u8>) -> u8,
{
    let normal = Normal::new(mean, std_dev).unwrap();
    img.pixels()
        .map(|pixel| {
            let value = get_channel(pixel) as f64;
            let noise_value = normal.sample(rng);
            (value + noise_value).clamp(0.0, 255.0) as u8
        })
        .collect()
}
