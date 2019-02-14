extern crate image;

use crate::utils;
use crate::web;

pub fn check_60_amp_status() {
    let img = web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc2.png")
        .expect("Unable to get image");

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
    let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(&pixels);

    if all_good == sum_of_good_cyrostats {
        println!("All PCPermits are up");
    } else {
        println!("A PCPermit is down");
    }
}
