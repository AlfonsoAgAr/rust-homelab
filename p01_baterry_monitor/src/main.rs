// Project 01
// File reading, error handling, type conversion

use std::fs; // Filesystem from standar lib

fn main() {
    let path = "/sys/class/power_supply/battery/capacity";

    println!("Consulting system path: {}", path);

    let raw_content = fs::read_to_string(path)
        .expect("ERROR: Can not read battery from file");
    
    let clean_content = raw_content.trim();

    let percentage: u8 = clean_content.parse()
        .expect("ERROR: Invalid number");

    println!("**********");
    println!("Battery level: {}%", percentage);
    println!("**********");

    if percentage < 20 {
        println!("Connect the charger!!!");
    } else {
        println!("Everything is ok");
    }
}
