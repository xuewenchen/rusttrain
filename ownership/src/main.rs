fn main() {
    let my_str = String::from("this is simple str");
    println!("my string value is = {}", my_str);

    let mut change_str = String::from("hello,");
    change_str.push_str(" chenxuewen");
    println!("change str is {}", change_str);

    let s1 = String::from("yes");
    let s2 = s1;
    // println!("s1 = {s1}");
    println!("s2 = {s2}");

    // get ownership
    fn_ownership(String::from("hello"));
    
    // return ownership
    let s = get_ownership();
    println!("get str = {}", s);

}

// get ownership
fn fn_ownership(s: String) {
    println!("get str = {}", s);
}

// return ownership
fn get_ownership() -> String {
    String::from("get ownership")
}