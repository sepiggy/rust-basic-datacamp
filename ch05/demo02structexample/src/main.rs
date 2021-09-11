#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Rectangle {
        width: 100,
        length: 200,
    };
    let rect_area = area(&rect);
    println!("{}", rect_area);
    println!("{:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
