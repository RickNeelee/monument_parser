use clap::{Arg, App};
use monument_parser::monument_parser;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("Monument Parser")
        .version("1.0")
        .author("Rick Neelee")
        .about("Parser for the register of cultural monuments of Ukraine.")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Sets the input file to use.")
            .takes_value(true))
        .get_matches();

    if let Some(file) = matches.value_of("file") {
        let mut file = File::open(file).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");

        match monument_parser::monument(&contents) {
            Ok((monument_type, monument_name, location, (first_year, second_year), purpose_types)) => {
                println!("Type of monument: {}", monument_type);
                println!("Name of the monument: {}", monument_name);
                if location != "Invalid input" {
                    println!("Location: {}", location);
                } else {
                    println!("Error: Invalid input for location");
                }
                println!("Dating: {}-{}", first_year, second_year);
                if !purpose_types.is_empty() {
                    let purposes = purpose_types.join(", ");
                    println!("Purpose type: {}", purposes);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } else {
        println!("No file provided.");
    }
}
