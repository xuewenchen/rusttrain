use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;

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

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // let p = Person{age: 12, name: String::from("hello")};
    // let a = Box::new(p);
    // get_box(a);
    // println!("{:?}", a);

    // let my_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{:?}", my_list);

    // test_defer();
    // test_defer2();
    test_drop1();
}

fn get_box(a: Box<Person>) {
    println!("{:?}", a);
}

fn test_defer() {
    let x = 5;
    let y = &x;
    println!("{x}");
    println!("{}", *y);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


fn test_defer2() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn test_drop1() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
