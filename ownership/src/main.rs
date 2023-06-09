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
    let mut s = get_ownership();
    println!("get str = {}", s);

    // use reference in function
    let str_len = get_str_len(&s);
    println!("the str len is = {}", str_len);

    // mut reference
    mut_str(&mut s);
    println!("after mut the str is = {}", s);

    // test str slice
    test_str_slice();

}

// get ownership
fn fn_ownership(s: String) {
    println!("get str = {}", s);
}

// return ownership
fn get_ownership() -> String {
    String::from("get ownership")
}

fn get_str_len(s: &String) -> usize {
    s.len()
}

fn mut_str(s: &mut String) {
    s.push_str(", nothing");
}

fn test_str_slice() {
    let s = "mystr is ok";
    println!("{s}");

    let s1 = &s[1..2];
    println!("{s1}");

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}