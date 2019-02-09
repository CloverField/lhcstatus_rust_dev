extern crate image;

use image::GenericImageView;
use std::fs;

pub fn get_pixels(coords: &[(u32, u32)], img: image::DynamicImage) -> Vec<[u8; 4]> {
    let mut pixels = Vec::new();
    for x in coords {
        pixels.push(img.get_pixel(x.0, x.1).data);
    }
    pixels
}

pub fn get_sum_of_good_cryostats(pixels: Vec<[u8; 4]>) -> usize {
    let mut sum_of_good_cyrostats = 0;
    for &t in pixels.iter() {
        if t[0] == 0 && t[1] == 255 && t[2] == 0 {
            sum_of_good_cyrostats += 255;
        }
    }
    sum_of_good_cyrostats
}

pub fn clean_up_image() -> std::io::Result<()> {
    fs::remove_file("./test.png")?;
    Ok(())
}
