#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

// tuple struct
#[derive(Debug)]
struct Color(u32, u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(size: u32) ->Rectangle {
        Rectangle { width: size, height: size }
    } 
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

    let a = (1, 2 ,3);

    let red = Color(1, 2, 3);
    println!("{:?}", red); 

    println!("{}", red.0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("{:?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    let a = Rectangle::new(12);
    println!("{:?}", a);    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}