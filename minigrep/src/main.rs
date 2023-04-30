use std::{env};
use std::fs;

fn main() {

    // get all parameters
    let parameters: Vec<String> = env::args().collect();

    let search_string = &parameters[1];
    let search_file = &parameters[2];

    println!("search_string = {}, search_file = {}", search_string, search_string);

    let content = fs::read_to_string(search_file).expect("should need to read the file");

    println!("content = {}", content);

}
