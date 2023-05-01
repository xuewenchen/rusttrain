use std::process::exit;
use std::{env, process};
use std::fs;

struct  Config {
    search_string: String,
    search_file: String,
}

// implement some method for config
impl Config {
    fn build(parameters: & Vec<String>) -> Result<Config, &'static str> {
        // add parameters len check
        if parameters.len() < 3 {
            return Err("the parameter length is less than 3");
        }
        let search_string = &parameters[1];
        let search_file = &parameters[2];
        Ok(Config{search_string: search_string.clone(), search_file: search_file.clone()})
    }
}

fn main() {

    // get all parameters
    let parameters: Vec<String> = env::args().collect();

    let config = Config::build(&parameters).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    println!("search_string = {}, search_file = {}", config.search_string, config.search_file);

    let content = fs::read_to_string(config.search_file).expect("should need to read the file");

    println!("content = {}", content);

}