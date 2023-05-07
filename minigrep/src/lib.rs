
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

    for line in search(&config.search_string, &content) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a >(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();

    for ele in contents.lines() {
        if ele.contains(query) {
            result.push(ele);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}