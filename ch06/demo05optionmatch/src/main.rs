// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("six = {:#?}", six);
//     println!("none = {:#?}", none);
// }

// fn plus_one(a: Option<i32>) -> Option<i32> {
//     match a {
//         None => None,
//         Some(a) => Some(a + 1),
//     }
// }
//

fn main() {
    let v = 3u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
