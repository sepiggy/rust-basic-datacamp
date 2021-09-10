// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("true");
//     } else {
//         println!("false");
//     }
// }


// fn main() {
//     let condition = true;
//     let number = if condition {5} else {6};
//     println!("The value of number is: {}", number);
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 { break counter * 2; }
//     };
//
//     println!("The result is: {}", result);
// }

// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number);
//
//         number = number - 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     for x in a.iter() {
//         println!("the value is: {}", x);
//     }
// }

fn main() {
    for x in (1..4).rev() {
        println!("{}!", x);
    }
    println!("LIFTOFF!");
}