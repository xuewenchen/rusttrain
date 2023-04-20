use std::fs::File;
use std::io::ErrorKind;

fn main() {
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
    get_upwrap_expetct();
}

fn get_upwrap() {
    let file_result = File::open("world.txt").unwrap();
}

fn get_upwrap_expetct() {
    let file_result = File::open("world.txt").expect("there is some bad things");
}