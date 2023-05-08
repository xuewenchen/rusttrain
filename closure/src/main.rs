use std::thread;

use closure::*;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // test closure none annotation
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));

    // test closure
    only_borrows();

    // test closure
    only_borrows2();
    
    // test closure move
    only_move();

    only_borrows3();
}

fn only_borrows() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn only_borrows2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn only_move() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}


fn only_borrows3() {
    let mut x = 10;
    println!("Before defining closure: {:?}", x);

    let mut borrows_mutably = || x + 1;
    borrows_mutably();
    println!("Before defining closure: {:?}", x);
}