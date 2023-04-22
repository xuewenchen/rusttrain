use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// article implement for Summary
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

// tweet implement for Summary
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound multiple
pub fn notify3(item: &(impl Summary + Display)) {

}

pub fn notify4<T: Summary + Display>(item: &T) {

}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Display,
{
    unimplemented!()
}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}