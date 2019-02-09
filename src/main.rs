extern crate reqwest;
extern crate image;

use std::fs;
use std::fs::File;
use std::io;
use image::GenericImageView;
use image::Pixel;

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
        Err(e) => println!("Error: {}", e)
    }
}

fn select_option(n: u32){
    match n {
        1 => check_cryo_status(),
        _ => println!("Select a valid option")
    }
}

fn get_image(url: &str) {
    let mut resp = reqwest::get(url).expect("request failed");
    let mut out = File::create("./test.png").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

fn clean_up_image() -> std::io::Result<()> {
    fs::remove_file("./test.png")?;
    Ok(())
}

enum Sectors {
    Sector12,
    Sector23,
    Sector34,
    Sector45,
    Sector56,
    Sector67,
    Sector78,
    Sector81,
}

fn check_cryo_status() {
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
        1 => get_sector_status(Sectors::Sector12),
        2 => get_sector_status(Sectors::Sector23),
        3 => get_sector_status(Sectors::Sector34),
        4 => get_sector_status(Sectors::Sector45),
        5 => get_sector_status(Sectors::Sector56),
        6 => get_sector_status(Sectors::Sector67),
        7 => get_sector_status(Sectors::Sector78),
        8 => get_sector_status(Sectors::Sector81),
        _ => println!("Please Select a valid sector")
    }
}

fn get_sector_status(sector: Sectors) {
    get_image("https://vistar-capture.web.cern.ch/vistar-capture/lhc2.png");
    let img = image::open("./test.png").expect("Unable to open image");
    let rgb = img.get_pixel(100, 100).to_rgb();
    for x in &rgb.data {
        println!("{}", x);
    }

    match sector {
        Sectors::Sector12 => {
            println!("Selected Sector 12")
        },
        Sectors::Sector23 => {
            println!("Selected Sector 23")
        },
        Sectors::Sector34 => {
            println!("Selected Sector 34")
        },
        Sectors::Sector45 => {
            println!("Selected Sector 45")
        },
        Sectors::Sector56 => {
            println!("Selected Sector 56")
        },
        Sectors::Sector67 => {
            println!("Selected Sector 67")
        },
        Sectors::Sector78 => {
            println!("Selected Sector 78")
        },
        Sectors::Sector81 => {
            println!("Selected Sector 81")
        }
    }
    clean_up_image().expect("Unable to clean up image");
}
