
use std::fs;
use std::error::Error;

pub struct  Config {
    search_string: String,
    search_file: String,
}

// implement some method for config
impl Config {
    pub fn build(parameters: & Vec<String>) -> Result<Config, &'static str> {
        // add parameters len check
        if parameters.len() < 3 {
            return Err("the parameter length is less than 3");
        }
        let search_string = &parameters[1];
        let search_file = &parameters[2];
        Ok(Config{search_string: search_string.clone(), search_file: search_file.clone()})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.search_file).expect("should need to read the file");

    println!("content = {}", content);

    Ok(())
}