use std::{fs::File, io::ErrorKind};

// fn main() {
//     let f = File::open("hello.txt");
//
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Error opening file {:?}", error);
//         }
//     };
//
// }
//
//

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Error creating file: {:?}", e),
//             },
//             other_error => panic!("Error opening the file: {:?}", other_error),
//         },
//     };

//     println!("{:?}", f);
// }

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Error creating file: {:?}", error);
//             })
//         } else {
//             panic!("Error opening file: {:?}", error);
//         }
//     });
// }
// 

fn main() {
    // let f =  File::open("hello.txt").unwrap();
    let f =  File::open("hello.txt").expect("无法打开文件hello.txt");
}
