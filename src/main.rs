extern crate image;
extern crate reqwest;

mod check_cryo;
mod lhc_status_options;
mod utils;
mod web;

use std::io;

fn main() {
    println!("What do you want to check?");
    println!("1. Cryo Status");
    println!("2. Individual Cryo Status");
    println!("3. 60 Amp PC Permit Status");
    println!("4. Individual 60 Amp PC Permit Status");
    println!("5. RF Status");
    println!("6. RF Status Individual");
    println!("7. Beam Dump Status");
    println!("8. Beam Dump Component Status");
    println!("9. Experiment Magnet Status");
    println!("10. Individual Experiment Magnet Status");
    println!("11. Page 1 Beam SMP Status");
    println!("12. Page 1 Individual SMP Beam Status");
    //println!("13. Perform OCR on LHC Vistar page"); //Comeback to later
    //println!("14. Perform OCR on LHC vistar page comments"); //Comeback to later

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_num = input.trim().parse();

    match input_num {
        Ok(n) => select_option(n),
        Err(e) => println!("Error: {}", e),
    }
}

fn select_option(n: u32) {
    match n {
        1 => check_cryo::check_cryo_status(),
        3 => check_60_amp_status(),
        _ => println!("Select a valid option"),
    }
}

fn check_60_amp_status() {
    web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc2.png");
    let img = image::open("./test.png").expect("Unable to open image");
    let coords = [
        (108, 403), //S12
        (203, 403), //S23
        (297, 403), //S34
        (392, 402), //S45
        (498, 402), //S56
        (595, 402), //S67
        (688, 403), //S78
        (772, 402), //S81
    ];

    let pixels = utils::get_pixels(&coords, img);
    let all_good = pixels.len() * 255;
    let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

    if all_good == sum_of_good_cyrostats {
        println!("All PCPermits are up");
    } else {
        println!("A PCPermit is down");
    }

    utils::clean_up_image().expect("Unable to clean up image");
}
