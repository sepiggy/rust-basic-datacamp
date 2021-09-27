use std::fmt::Display;

// use std::fmt::Display;

// pub trait Summary {}

// pub struct NewsArticle {}

// impl Summary for NewsArticle {}

// pub struct Tweet {}

// impl Summary for Tweet {}

// // pub fn notify(item: impl Summary) {
// // }

// // pub fn notify<T: Summary>(item: T) {
// // }

// // 指定多个Trait
// // pub fn notify<T: Summary + Display>(item: T) {}

// // pub fn notify<T, U>(a: T, b: U) -> String where T: Summary + Display, U: Clone + Debug {}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T:Display+PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {}
