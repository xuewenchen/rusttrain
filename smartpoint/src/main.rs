use crate::List::{Cons, Nil};

#[derive(Debug)]
struct Person {
    age: u32,
    name: String,
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // let p = Person{age: 12, name: String::from("hello")};
    // let a = Box::new(p);
    // get_box(a);
    // println!("{:?}", a);

    let my_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", my_list);
}

fn get_box(a: Box<Person>) {
    println!("{:?}", a);
}