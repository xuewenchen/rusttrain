use testgeneric::{Summary, Tweet, NewsArticle, longest};


// generic struct with one parameters
struct Point<T> {
    x: T,
    y: T,
}

// define method use generic parameter
impl <T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

// can just define simple concrete type also is valid
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// generci struct with two parameters
struct Point2<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Dangling<'a> {
    name: &'a str,
}

// #[derive(Debug)]
// struct Dangling2{
//     name: &str,
// }


fn main() {
    
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    test_longest_with_ok();
    test_longest_with_fail();

    let s = String::from("hello world");
    let p = Dangling {name: &s};
    println!("{:?}", p);

    let a: &'static str = "hello world";
    println!("{a}");
}

fn test_reference_change() {
    let a = 10;
    let b = 20;
    let mut c = &a;
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");

    c = &b;
    println!("c = {c}");
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    

    largest
}

// generic function
fn get_largest_item<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for ele in list {
        if ele > largest {
            largest = ele;
        }
    }
    largest
}

fn test_longest_with_ok() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn test_longest_with_fail() {
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}