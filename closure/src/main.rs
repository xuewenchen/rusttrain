use std::thread;

use closure::*;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

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

    // let mut a = None;

    // let x = a.unwrap_or_else(|| 10);
    // a.unwrap_or_else(|| 20);
    // println!("{}", x);

    // sort_list();

    // sort_list2();

    sort_list3();


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

fn sort_list() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn sort_list2() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut counter = 0;
    

    list.sort_by_key(|r| {
        counter += 1;
        r.width
    });
    println!("{:#?}", list);
    
    println!("total count = {}", counter);
}

fn sort_list3() {
    let mut v1 = vec![1, 2, 3];
    for ele in &mut v1 {
        *ele = 10;
    }

    for ele in v1 {
        println!("{ele}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        println!("{total}");
    }

    #[test]
    fn test_map() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
        for ele in v2 {
            println!("{ele}");
        }
    }
}