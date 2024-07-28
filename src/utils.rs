use image::GrayImage;

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
