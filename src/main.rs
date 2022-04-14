
use glam::*;
use image::{RgbImage, Rgb};

fn main() {

    let width = 256;
    let height = 256;
    let mut img = RgbImage::new(width, height);
    
    for x in 0..width {
        for y in 0..height {

            let r = x as f32 / (width as f32 - 1.0);
            let g = (height  - y) as f32 / (height as f32 - 1.0);
            let b = 0.25f32;

            img.put_pixel(x, y,  Rgb(
                [
                    (r * 255.999) as u8,
                    (g * 255.999) as u8,
                    (b * 255.999) as u8
                ])
            )
        }
    }
    img.save("out.png").unwrap();
}
