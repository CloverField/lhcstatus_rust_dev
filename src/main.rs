extern crate image;
extern crate reqwest;

mod check_beam_dump;
mod check_cryo;
mod check_exp_magnets;
mod check_pcpermits;
mod check_rf;
mod check_smp;
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
        3 => check_pcpermits::check_60_amp_status(),
        5 => check_rf::check_rf_status(),
        7 => check_beam_dump::check_beam_dump_status(),
        9 => check_exp_magnets::check_exp_magnet_status(),
        11 => check_smp::check_smp_status(),
        _ => println!("Select a valid option"),
    }
}
