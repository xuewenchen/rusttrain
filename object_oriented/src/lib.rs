#[derive(Debug)]
pub struct Human {
    pub age: u32,
    pub name: String,
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for item in self.components.iter() {
            item.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("this is in button")
    }
}

pub struct SelectBox {
    pub  width: u32,
    pub  height: u32,
    pub  options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("this is in selectBox")
    }
}

