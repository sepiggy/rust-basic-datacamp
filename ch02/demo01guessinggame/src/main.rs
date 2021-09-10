use std::{io, cmp}; // prelude
use rand::Rng;

fn main() {
    println!("猜数游戏！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // io::Result Ok, Err
        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("太小了!"),
            cmp::Ordering::Greater => println!("太大了!"),
            cmp::Ordering::Equal => {
                println!("你赢了!");
                break;
            },
        }
    }
}
