use std::process::exit;
use std::{env, process};
use std::error::Error;


use minigrep::Config;

fn main() {

    // get all parameters
    let parameters: Vec<String> = env::args().collect();

    let config = Config::build(&parameters).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    minigrep::run(config);
}

