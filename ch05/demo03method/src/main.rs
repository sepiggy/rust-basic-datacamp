#[derive(Debug)]
struct Rect {
    width: u32,
    length: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.length > other.length
    }
}

impl Rect {
    // 关联函数
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        length: 50,
    };

    println!("{}", rect.area());
    println!("{:#?}", rect);

    let rect1 = Rect {
        width: 100,
        length: 200,
    };

    let rect2 = Rect {
        width: 110,
        length: 90,
    };

    println!("{}", rect1.can_hold(&rect2));

    let square = Rect::square(100);
    println!("{:#?}", square);
}
