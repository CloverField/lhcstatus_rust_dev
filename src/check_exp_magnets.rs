extern crate image;

use crate::utils;
use crate::web;

pub fn check_exp_magnet_status() {
    get_exp_magnet_status();
}

fn get_exp_magnet_status() {
    let img = web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc2.png")
        .expect("Unable to get image");

    let coords = [
        (365, 60),  //ALICE solenoid
        (365, 100), //ALICE dipole
        (365, 140), //ATLAS solenoid
        (365, 180), //ATLAS toroid
        (365, 220), //CMS solenoid
        (365, 260), //LHCb dipole
    ];

    let pixels = utils::get_pixels(&coords, img);
    let all_good = 255 * pixels.len();
    let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

    if all_good == sum_of_good_cyrostats {
        println!("All Exp Magnets are functioning correctly");
    } else {
        println!("Not all Exp Magnets are functioning correctly");
    }
}
