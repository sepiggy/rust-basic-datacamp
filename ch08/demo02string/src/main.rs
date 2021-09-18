// fn main() {
//     let s = String::new();
//     s.push_str("hello");
//     println!("{}", s);
// }

// fn main() {
//     let data = "initial contents";
//     let s = data.to_string();
// 	println!("{}", s);
//
//     let s1 = "initial contents".to_string();
// 	println!("{}", s1);
//
// 	let s = String::from("hello");
// 	println!("{}", s);
// }

// fn main() {
//     let mut s = String::from("foo");
//     let s1 = String::from("bar");
//     s.push_str(&s1);
//     println!("{}", s1);
// }

// fn main() {
//     let mut s = String::from("lo");
//     s.push('l');
//     println!("{}", s);
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World!");
//
//     let s3 = s1 + &s2;
//     println!("{}", s3);
//
// 	println!("{}", s1);
// 	println!("{}", s2);
// }

// fn main() {
// 	let s1 = String::from("tic");
// 	let s2 = String::from("tac");
// 	let s3 = String::from("toe");
//
// 	let result = format!("{}-{}-{}", s1, s2, s3);
// 	println!("{}", result);
//
// 	println!("{}, {}, {}", s1, s2, s3);
// }

fn main() {
    let len = String::from("Hola").len();
    println!("{}", len);

    let s = "sepiggy";

    for i in s.chars() {
        println!("{}", i);
    }

    for i in s.bytes() {
        println!("{}", i);
    }
}
