fn main() {
    let s = String::from("Hello World");

    take_ownership(s);

    // println!("s: {}", s);

    let x = 5;

    make_copy(x);

    println!("x: {}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
}