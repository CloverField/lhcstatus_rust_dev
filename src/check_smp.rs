extern crate image;

use crate::lhc_status_options::Beams;
use crate::utils;
use crate::web;

use std::io;

pub fn check_smp_status() {
    println!("Which Beam do you want to check?");
    println!("1. Beam 1");
    println!("2. Beam 2");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Unable to convert to u32");

    match input {
        1 => get_smp_status(Beams::Beam1),
        2 => get_smp_status(Beams::Beam2),
        _ => println!("Please select a valid beam"),
    }
}

fn get_smp_status(beam: Beams) {
    let img = web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc1.png")
        .expect("Unable to get image");

    match beam {
        Beams::Beam1 => {
            let coords = [
                (872,572),  //Stable Beams
                (872,600),  //Moveable Devices Allowed In
                (872,629),  //Beam Presence
                (872,658),  //Setup Beam
                (872,686),  //Global Beam Permit
                (872,715)   //Link Status of Beam Permits
            ];

            let mut pixels = utils::get_pixels(&coords, img);

            if pixels[5][1] == 255 {
                pixels.swap_remove(3); //if in stable beams remove the setup flag
            } else if pixels.len() == 6 && pixels[3][1] == 255 {
                pixels.swap_remove(5); //if in setup remove the stable beams flag
            }

            let all_good = 255 * pixels.len();
            let sum_of_good_components = utils::get_sum_of_good_components(pixels);

            if all_good == sum_of_good_components {
                println!("Beam 1's SMP status is good");
            } else {
                println!("There is a fault with Beam 1's SMP status");
            }
        }
        Beams::Beam2 => {
            let coords = [
                (945,572),  //Stable Beams
                (945,600),  //Moveable Devices Allowed in
                (945,629),  //Beam Presence
                (945,658),  //Setup Beam
                (945,686),  //Global Beam Permit
                (945,715)   //Link Status of Beam Permits
            ];

            let mut pixels = utils::get_pixels(&coords, img);

            if pixels[5][1] == 255 {
                pixels.swap_remove(3); //if in stable beams remove the setup flag
            } else if pixels.len() == 6 && pixels[3][1] == 255 {
                pixels.swap_remove(5); //if in setup remove the stable beams flag
            }

            let all_good = 255 * pixels.len();
            let sum_of_good_components = utils::get_sum_of_good_components(pixels);

            if all_good == sum_of_good_components {
                println!("Beam 2's SMP status is good");
            } else {
                println!("There is a fault with Beam 2's SMP status");
            }
        }
    }
}