#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

fn main() {
    let p1 = Person {
        name: String::from("hello"),
        age: 12,
        address: String::from("world"),
    };
    let p2 = Person {
        name: String::from("hello"),
        address: p1.address.clone(),
        ..p1
    };

    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);

    println!("Hello, world!");
}
