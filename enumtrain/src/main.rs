use core::num;

#[derive(Debug)]
enum IpAddr {
    IpV4(String),
    IpV6(String),
}

impl IpAddr {
    fn call(&self) {
        println!("call");
    }
}

fn main() {
    let v4 = IpAddr::IpV4(String::from("this is ip4"));
    let v6 = IpAddr::IpV6(String::from("this is ip6"));

    println!("v4 = {:?}", v4);
    println!("v6 = {:?}", v6);
    v4.call();

    // Option
    let a = Some(1);
    let b = Some("123");
    let c: Option<i32> = None;

    let x = Some(1);
    if let Some(max) = x {
        println!("the max value is {}", max);
    }
    let q = if let Some(max) = x {
        max
    } else {
        1
    };
    println!(" q = {q}");

}


fn router(ip_type: IpAddr) -> bool {
    true
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn get_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1, 
        Coin::Nickel => 1, 
        Coin::Dime => 1, 
        Coin::Quarter(state) => {
            println!("state = {:?}", state);
            0
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn add_number_to_optional(number: Option<u32>) -> Option<u32> {
    match number {
        None => None,
        Some(x) => Some(x + 1),
    }
}