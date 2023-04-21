use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    // get_upwrap();
    // get_upwrap_expetct();

    
    Ok(())
}

fn get_upwrap() {
    let file_result = File::open("world.txt").unwrap();
}

fn get_upwrap_expetct() {
    let file_result = File::open("world.txt").expect("there is some bad things");
}

fn get_error_from_fn() -> Result<String, std::io::Error> {
    let file_handler_result = File::open("hello.txt");
    let mut file_handler = match file_handler_result {
        Ok(handler) => handler,
        Err(e) => return Err(e)
    };
    let mut username = String::new();

    match file_handler.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn get_error_from_fn_simple() -> Result<String, std::io::Error> {
    let mut file_handler = File::open("hello.txt")?;
    let mut username = String::new();
    file_handler.read_to_string(&mut username)?;
    Ok(username)
}

fn get_error_from_fn_simple2() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn get_error_from_fn_simple3() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}