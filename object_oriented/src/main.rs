use object_oriented::Human;
use object_oriented::*;


#[derive(Debug)]
pub struct Person {
    age: u32,
    name: String,
}

fn main() {
    let mut p = Person{ age: 12, name: String::from("hello")};
    p.age = 13;
    println!("{:?}", p);

    let h = Human{
        age: 12,
        name: String::from("hello")
    };
    println!("{:?}", h);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    println!("{}", a);
    println!("{}", b);
}

struct Point {
    x: i32,
    y: i32,
}