extern crate image;

use crate::lhc_status_options::CryoSectors;
use crate::utils;
use crate::web;

use std::io;

pub fn check_cryo_status() {
    println!("Which sector do you want to check?");
    println!("1. Sector 12");
    println!("2. Sector 23");
    println!("3. Sector 34");
    println!("4. Sector 45");
    println!("5. Sector 56");
    println!("6. Sector 67");
    println!("7. Sector 78");
    println!("8. Sector 81");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Unable to convert to u32");

    match input {
        1 => get_sector_status(CryoSectors::Sector12),
        2 => get_sector_status(CryoSectors::Sector23),
        3 => get_sector_status(CryoSectors::Sector34),
        4 => get_sector_status(CryoSectors::Sector45),
        5 => get_sector_status(CryoSectors::Sector56),
        6 => get_sector_status(CryoSectors::Sector67),
        7 => get_sector_status(CryoSectors::Sector78),
        8 => get_sector_status(CryoSectors::Sector81),
        _ => println!("Please Select a valid sector"),
    }
}

fn get_sector_status(sector: CryoSectors) {
    let img = 
        web::get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc2.png")
        .expect("Unable to get image");

    match sector {
        CryoSectors::Sector12 => {
            let coords = [
                (100, 100), //CMITR1
                (188, 100), //CSITR1
                (288, 100), //CMMSR1
                (378, 100), //CSMSR1
                (478, 100), //CMAR12
                (568, 100), //CSAR12
                (668, 100), //CMMSL2
                (758, 100), //CSMSL2
                (858, 100), //CMITL2
                (948, 100), //CSITL2
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 12");
            } else {
                println!("Cyro is down in Sector 12");
            }
        }
        CryoSectors::Sector23 => {
            let coords = [
                (100, 140), //CMITR2
                (188, 140), //CSITR2
                (288, 140), //CMMSR2
                (378, 140), //CSMSR2
                (478, 140), //CMAML3
                (568, 140), //CSAML3
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 23");
            } else {
                println!("Cyro is down in Sector 23");
            }
        }
        CryoSectors::Sector34 => {
            let coords = [
                (478, 175), //CMAML3
                (568, 175), //CSAML3
                (668, 175), //CMMSL1
                (758, 175), //CSMSL1
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 34");
            } else {
                println!("Cyro is down in Sector 34");
            }
        }
        CryoSectors::Sector45 => {
            let coords = [
                (288, 210), //CMMSR4
                (378, 210), //CSMSR4
                (478, 210), //CMAR45
                (568, 210), //CSAR45
                (668, 210), //CMMSL5
                (758, 210), //CSMSL5
                (858, 210), //CMITL6
                (948, 210), //CSITL6
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 45");
            } else {
                println!("Cyro is down in Sector 45");
            }
        }
        CryoSectors::Sector56 => {
            let coords = [
                (100, 245), //CMITR5
                (188, 245), //CSITR5
                (288, 245), //CMMSR5
                (378, 245), //CSMSR5
                (478, 245), //CMAR56
                (568, 245), //CSAR56
                (668, 245), //CMMSL6
                (758, 245), //CSMSL6
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 56");
            } else {
                println!("Cyro is down in Sector 56");
            }
        }
        CryoSectors::Sector67 => {
            let coords = [
                (288, 280), //CMMSR6
                (378, 280), //CSMSR6
                (478, 280), //CMAML7
                (568, 280), //CSAML7
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 67");
            } else {
                println!("Cyro is down in Sector 67");
            }
        }
        CryoSectors::Sector78 => {
            let coords = [
                (478, 315), //CMAMR7
                (568, 315), //CSAMR7
                (668, 315), //CMMSL8
                (758, 315), //CSMSL8
                (858, 315), //CMITL8
                (948, 315), //CSITL8
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 78");
            } else {
                println!("Cyro is down in Sector 78");
            }
        }
        CryoSectors::Sector81 => {
            let coords = [
                (100, 350), //CMITR8
                (188, 350), //CSITR8
                (288, 350), //CMMSR8
                (378, 350), //CSMSR8
                (478, 350), //CMAR81
                (568, 350), //CSAR81
                (668, 350), //CMMSL1
                (758, 350), //CSMSL1
                (858, 350), //CMITL1
                (948, 350), //CSITL1
            ];

            let pixels = utils::get_pixels(&coords, img);
            let all_good = 255 * pixels.len();
            let sum_of_good_cyrostats = utils::get_sum_of_good_cryostats(pixels);

            if all_good == sum_of_good_cyrostats {
                println!("Everything looks good in Sector 81");
            } else {
                println!("Cyro is down in Sector 81");
            }
        }
    }
    //utils::clean_up_image().expect("Unable to clean up image");
}
