// fn main() {
//     let s = "hello";
//     let mut s = String::from("Hello");
//     s.push_str(", World");
//     println!("s={}", s);
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = s1;
//
//     println!("{}", s1);
// }

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);
}