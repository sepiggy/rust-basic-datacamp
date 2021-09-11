use pause_console::*;

// fn main() {
//     let mut s = String::from("Hello world");
//     let word_index = first_word(&s);
//     println!("{}", word_index);
//     pause_console!()
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
    
//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
// 

// fn main() {
//     let s = String::from("Hello world");
//     let hello = &s[..5];
//     let world = &s[6..];
//     println!("{}, {}", hello, world);
    
//     let whole = &s[..];
//     println!("{}", whole);
    
//     let s1 = String::from("sepiggy");
//     println!("{}", &s1[0..3]);
//     pause_console!();
// }
// 

// fn main() {
//     let s= String::from("Helloworld");
//     let word_index = first_world(&s);
//     // s.clear();
//     println!("{}", word_index);
//     pause_console!();
// }

// fn first_world(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }
// 

fn main() {
    let my_string = String::from("Hello world");
    let word_index = first_word(&my_string[..]);
    println!("{}", word_index);
    
    let my_string_literal = "hello world";
    let word_index = first_word(my_string_literal);
    println!("{}", word_index);
    pause_console!();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}