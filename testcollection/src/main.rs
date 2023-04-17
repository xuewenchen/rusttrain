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
    for ele in &b {
        println!("ele = {ele}");
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
    
}
