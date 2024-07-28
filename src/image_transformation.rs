use image::{DynamicImage, GenericImageView, RgbaImage};

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
