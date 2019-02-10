extern crate image;

use crate::lhc_status_options::Beams;
use crate::utils;
use crate::web;

use std::io;

pub fn check_beam_dump_status() {
    println!("Which Beam do you want to check?");
    println!("1. Beam 1");
    println!("2. Beam 2");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Unable to convert to u32");

    match input {
        1 => get_beam_dump_status(Beams::Beam1),
        2 => get_beam_dump_status(Beams::Beam2),
        _ => println!("Please select a valid beam"),
    }
}

fn get_beam_dump_status(beam: Beams) {
    let img = web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhcbds.png")
        .expect("Unable to get image");

    match beam {
        Beams::Beam1 => {
            let coords = [
                (192, 38),  //BeamOneDumped
                (73, 60),   //Kicker
                (200, 60),  //BETS
                (323, 60),  //IPOC - Beam Dump Pane
                (76, 80),   //LASS
                (200, 82),  //RETRIGGER
                (326, 82),  //XPOC
                (81, 101),  //REMOTE - Beam Dump Pane
                (193, 102), //ON - Beam Dump Pane
                (90, 168),  //REMOTE - Injection Pane
                (194, 168), //ON - Injection Pane
                (333, 168), //TIMING ON
                (65, 189),  //CONDITIONING
                (286, 188), //TIMEOUT
                (111, 210), //IPOC - Injection Pane
                (290, 210), //IQC
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_components = utils::get_sum_of_good_components(pixels);

            if all_good == sum_of_good_components {
                println!("Everything looks good for the Beam 1 Beam Dump");
            } else {
                println!("Looks like there is an error with the Beam 2 Beam Dump");
            }
        }
        Beams::Beam2 => {
            let coords = [
                (593, 38),  //BeamTwoDumped
                (472, 60),  //Kicker
                (600, 60),  //BETS
                (723, 60),  //IPOC - Beam Dump Pane
                (476, 80),  //LASS
                (600, 82),  //RETRIGGER
                (726, 82),  //XPOC
                (481, 101), //REMOTE - Beam Dump Pane
                (593, 102), //ON - Beam Dump Pane
                (490, 168), //REMOTE - Injection Pane
                (594, 168), //ON - Injection Pane
                (733, 168), //TIMING ON
                (465, 189), //CONDITIONING
                (686, 188), //TIMEOUT
                (511, 210), //IPOC - Injection Pane
                (690, 210), //IQC
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_components = utils::get_sum_of_good_components(pixels);

            if all_good == sum_of_good_components {
                println!("Everything looks good for the Beam 2 Beam Dump");
            } else {
                println!("Looks like there is an error with the Beam 2 Beam Dump");
            }
        }
    }
}
