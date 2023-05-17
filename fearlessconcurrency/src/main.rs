use std::thread;
use std::time::Duration;
use std::sync::mpsc;


#[derive(Debug)]
struct Person {

}

fn main() {
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // // drop(v); // oh no!

    // println!("{v}");

    // handle.join().unwrap();

    test_mpsc();
}


fn test_mpsc() {
    let (tx, rx) = mpsc::channel();

    let tx_handler = thread::spawn(move|| {
        for i in 1..10 {
            let age = String::from("hello");
            tx.send(age).unwrap();
            // println!("{}", age);
        }
        
    });

    let rx_handler = thread::spawn(move || {
        for i in 1..10 {
            let x = rx.recv().unwrap();
            println!("{x}");
        }
    });

    tx_handler.join().unwrap();
    rx_handler.join().unwrap();
}