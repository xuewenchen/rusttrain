fn main() {
    
    find_largest_num();
}

fn find_largest_num() {
    let list = vec![1, 2, 10, 4, 5];
    let mut flag_number = &list[0];

    for ele in &list {
        if ele > flag_number {
            flag_number = ele;
        }
    }
    println!("largest number is {flag_number}");
    for ele in &list {
        println!("number is {ele}");
    }   
}
