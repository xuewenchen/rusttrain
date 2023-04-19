fn main() {
    // vec
    let mut a: Vec<i32> = Vec::new();
    a.push(11);
    a.push(12);
    a.push(13);
    a.push(14);

    for ele in a {
        println!("ele = {}", ele);
    }

    println!(" ========== this is just a ========= ");

    // new vec use vec!
    let mut b = vec![1, 2, 3];
    for ele in &mut b {
        *ele = 10;
        println!("ele1 = {ele}");
    }
    for ele in &mut b {
        println!("ele2 = {ele}");
    }

    // get data from vec
    let item = b.get(10);
    let a = match item {
        Some(i) => i,
        None => &0,
    };
    println!("a = {a}");

    if let Some(yy) = item {
        println!("there is something in item = {yy}");
    } else {
        println!("there is noting in item");
    }


    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("s1");
    let s2 = String::from("s2");

    let s3 = s1 + &s2;
    println!("s3 = {s3}");
    let ss = &s2[..];
    // println!("s1 = {s1}");
    // println!("s2 = {s2}");

    let s1 = String::from("hello1");
    let s2 = String::from("hello2");
    let s3 = String::from("hello3");
    let s4 = format!("{s1}--{s2}--{s3}");
    println!("s4 = {s4}");

    for c in "Здравствуйте".chars() {
        println!("{c}");
    }
    for c in "abc123陈学文".chars() {
        println!("{c}");
    }

}
