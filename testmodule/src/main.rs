use crate::garden::Person;

fn main() {
    println!("Hello, world!");
    let p = Person::new();
    println!("person is {:?}", p);
}

mod garden {
    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub age: u32,
    }

    impl Person {
        pub fn new() -> Person {
            Person { name: String::from("hello"), age: 111 }
        }
    }
}