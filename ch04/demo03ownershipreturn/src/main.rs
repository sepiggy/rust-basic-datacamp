use pause_console::*;

// fn main() {
//     let s1 = gives_ownership();
    
//     let s2 = String::from("hello");
    
//     let s3 = takes_and_give_back(s2);
// }


// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_give_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    let s1 = String::from("hello");
    
    let (s2, len) = calc_length(s1);
    
    println!("The length of '{}' is {}.", s2, len);
    
    pause_console!();
}

fn calc_length(s:String) -> (String, usize) {
    let length = s.len();
    
    (s, length)
}
