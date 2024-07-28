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
