use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guss the number!");

    println!("Please input you number!");

    let number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {number}");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&number) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal => println!("equal"),
    }
}
