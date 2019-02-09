extern crate reqwest;

use std::fs::File;
use std::io;

pub fn get_image(url: &str) {
    let mut resp = reqwest::get(url).expect("request failed");
    let mut out = File::create("./test.png").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}