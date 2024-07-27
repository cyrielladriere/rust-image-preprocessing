use image::{DynamicImage, GenericImageView, GrayImage};

// Calculate the average brightness of an image,
// returned as a float between 0 and 1.
fn average_brightness(raw_rgb: Vec<u8>) -> f64 {
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

fn calc_blur(img: DynamicImage) -> f64 {
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

fn calculate_variance(image: &Vec<f64>) -> f64 {
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

fn apply_laplacian(image: &GrayImage) -> Vec<f64> {
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

fn main() {
    let dir: &str = "images/pulp_fiction.png";
    let img: DynamicImage = image::open(dir).unwrap();
    // let (width, height) = img.dimensions();
    // let pix = img.get_pixel(0, 0);
    println!("dimensions: {:?}", img.dimensions());

    let brightness: f64 = average_brightness(img.to_rgb8().to_vec());
    println!("brightness: {:?}", brightness);

    let blur_score = calc_blur(img);
    println!("blur_score: {:?}", blur_score);
}
