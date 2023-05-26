extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn hello() {
    println!("this is a dangerous function must be called in unsafe block!");
}

fn main() {
    // unsafe { hello() };

    // let x = 10;
    // let xp = &x as *const i32;
    // unsafe {
    //     println!("{}", *xp);
    // }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

