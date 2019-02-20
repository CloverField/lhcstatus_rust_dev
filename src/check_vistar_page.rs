extern crate image;
extern crate tesseract_wrapper;

use crate::lhc_status_options::Vistar;
use crate::web;

use std::io;

pub fn check_vistar_status() {
    println!("What do you want to check on Page1");
    println!("1. The entire page");
    println!("2. The current comments");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Unable to convert to u32");

    match input {
        1 => get_vistar_status(Vistar::EntirePage),
        2 => get_vistar_status(Vistar::Comments),
        _ => println!("Please select a valid beam"),
    }
}

fn get_vistar_status(vistar: Vistar) {
    let mut img = web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc1.png")
        .expect("Unable to get image");

    match vistar {
        Vistar::EntirePage => {
            let output = tesseract_wrapper::run_tesseract_get_result(img, "eng", "", "");
            println!("{}", output);
        }
        Vistar::Comments => {
            let cropped_img = img.crop(0,556,512,731);
            let output = tesseract_wrapper::run_tesseract_get_result(cropped_img, "eng", "", "");
            println!("{}", output);
        }
    }
}
