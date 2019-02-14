extern crate image;

use crate::lhc_status_options::RFSectors;
use crate::utils;
use crate::web;

use std::io;

pub fn check_rf_status() {
    println!("Which sector do you want to check?");
    println!("1. Sector 1L4");
    println!("2. Sector 1R4");
    println!("3. Sector 2L4");
    println!("4. Sector 2R4");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Unable to convert to u32");

    match input {
        1 => get_rf_status(RFSectors::Sector1L4),
        2 => get_rf_status(RFSectors::Sector1R4),
        3 => get_rf_status(RFSectors::Sector2L4),
        4 => get_rf_status(RFSectors::Sector2R4),
        _ => println!("Please Select a valid sector"),
    }
}

fn get_rf_status(sector: RFSectors) {
    let img = web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc2.png")
        .expect("Unable to get image");

    match sector {
        RFSectors::Sector1L4 => {
            let coords = [
                (100, 440), //CM1L4
                (188, 440), //CS1L4
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(&pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 1L4");
            } else {
                println!("Cyro is down in Sector Sector 1L4");
            }
        }
        RFSectors::Sector1R4 => {
            let coords = [
                (480, 440), //CM1R4
                (570, 440), //CS1R4
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(&pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 1R4");
            } else {
                println!("Cyro is down in Sector Sector 1R4");
            }
        }
        RFSectors::Sector2L4 => {
            let coords = [
                (290, 440), //CM2L4
                (380, 440), //CS2L4
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(&pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 1R4");
            } else {
                println!("Cyro is down in Sector Sector 1R4");
            }
        }
        RFSectors::Sector2R4 => {
            let coords = [
                (670, 440), //CM2R4
                (760, 440), //CS2R4
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(&pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 1R4");
            } else {
                println!("Cyro is down in Sector Sector 1R4");
            }
        }
    }
}
