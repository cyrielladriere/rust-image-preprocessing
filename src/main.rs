use image::GenericImageView;

/// Calculate the average brightness of an image,
/// returned as a float between 0 and 1.
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

fn main() {
    let dir = "image.png";
    let img = image::open(dir).unwrap();
    // let (width, height) = img.dimensions();
    // let pix = img.get_pixel(0, 0);
    println!("{:?}", img.dimensions());

    let brightness = average_brightness(img.to_rgb8().to_vec());
    println!("{:?}", brightness);
}
