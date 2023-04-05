// const
const X: u32 = 100;

fn main() {
    println!("x = {X}");

    // immutable
    let a = 10;
    println!("a = {a}");

    // mutable
    let mut b = 10;
    println!("b = {b}");
    b = 20;
    println!("b after change = {b}");

    // shadow
    let a = "hello";
    println!("a before change = {a}");
    let a = 10;
    println!("a after change = {a}");

    // interge
    let inl = 10;
    println!("inl {inl}");

    // float
    let fl: f32 = 1.2;
    println!("fl {fl}");

    // bool
    let yes: bool = true;
    println!("yes {yes}");

    // character
    let ch = 'z';
    println!("ch {ch}");


    // tuple
    let mytuple = (1, "1", 1.2);
    println!("mytuple = {:?}", mytuple);
    let (x, y, z) = mytuple;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // array
    let myarr = [1, 2, 3];
    println!("myarr = {:?}", myarr);

    // call echo function
    echo_without_parameters();

    // call echo function
    echo_with_parameters(String::from("chenxuewen"));

    // expression
    let x = {
        let a = 10;
        a + 1
    };
    println!("expression result is {x}");

    // return value by expression
    let x = get_value();
    println!("get value by expression result is {x}");

    // learn control flow
    learn_control_flow();

}

// echo without parameters
fn echo_without_parameters() {
    println!("this is a function in rust");
}

// echo function with parameters
fn echo_with_parameters(name: String) {
    println!("your name is {name}");
}

// function return value by expression
fn get_value() -> u32 {
    5
}

// learn control flow in rust
fn learn_control_flow() {
    // if expression
    let a = 10;
    if a > 10 {
        println!("a is > 10");
    } else if a < 10 {
        println!("a is < 10");
    } else {
        println!("a is = 10");
    }

    // use if expression to let
    let condition = true;
    let a = if condition { 5 } else { 6 };
    println!("use if expression to let {a}");


    // loop expression
    let mut counter = 0;
    let result = loop {
        if counter < 10 {
            counter += 1;
        } else {
            break counter * 2
        }
    };
    println!("loop expression result = {result}");

    // while
    let mut a = 10;
    while a > 0 {
        println!("a is ok now = {a}");
        a -= 1;
    }
    println!("while is ending");

    let arr = [1, 2, 3, 4, 5];
    for ele in arr {
        println!("arr now ele = {ele}");
    }
    println!("for in collection is ended");
}