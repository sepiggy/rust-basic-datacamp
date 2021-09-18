use std::collections::HashMap;

// fn main() {
// 	let mut scores: HashMap<String, i32> = HashMap::new();
//
// 	scores.insert(String::from("Blue"), 10);
// 	scores.insert(String::from("yellow"), 50);
// }

// fn main() {
// 	let teams = vec![String::from("Blue"), String::from("Yellow")];
// 	let intial_scores = vec![10, 50];
//
// 	let scores: HashMap<_, _> =
// 		teams.iter().zip(intial_scores.iter()).collect();
//
// 	println!("{:#?}", scores);
// }

// fn main() {
// 	let field_name = String::from("Favorite color");
// 	let field_value = String::from("Blue");
//
// 	let mut map = HashMap::new();
// 	map.insert(&field_name, &field_value);
//
// 	println!("{}: {}", field_name, field_value);
// }

// fn main() {
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);
//
//     match score {
//         Some(s) => println!("{}", s),
//         None => println!("team not exist"),
//     }
// }

// fn main() {
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//
//     for (k, v) in &scores {
//         println!("{}: {}", k, v);
//     }
//
//     println!("{:#?}", scores);
// }

// fn main() {
// 	 let mut scores = HashMap::new();
//
// 	 scores.insert(String::from("Blue"), 10);
// 	 scores.insert(String::from("Blue"), 100);
//
// 	 println!("{:?}", scores);
// }

// fn main() {
// 	let mut scores = HashMap::new();
// 	scores.insert(String::from("Blue"), 10);
//
// 	let e = scores.entry(String::from("Yellow"));
// 	println!("{:#?}", e);
// 	e.or_insert(50);
//
// 	scores.entry(String::from("Blue")).or_insert(50);
//
// 	println!("{:#?}", scores);
// }

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
