pub fn hello() {
    println!("hello world human!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }

    #[test]
    fn test_hello2() {
        hello();
    }
}