extern crate reqwest;

use std::io;
use std::fs;
use std::fs::File;


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

    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    println!("{}", input);
}

fn get_image(url: &str){
    let mut resp = reqwest::get(url).expect("request failed");
    let mut out = File::create("./test.png").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

fn clean_up_image() -> std::io::Result<()>{
    fs::remove_file("./test.png")?;
    Ok(())
}

#[derive(Debug)]
enum Sectors{
    Sector12,
    Sector23,
    Sector34,
    Sector45,
    Sector56,
    Sector67,
    Sector78,
    Sector81
}