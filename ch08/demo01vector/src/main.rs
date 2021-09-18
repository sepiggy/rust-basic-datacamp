// fn main() {
//     let v: Vec<i32> = Vec::new();
//     let v = vec![1, 2, 3];
//     let mut v = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.push(4);
//     v.push(5);
//     println!("{:#?}", v);
// }

// fn main() {
// 	let v = vec![1,2,3,4,5];
// 	let third = &v[2];
// 	println!("The third element is {}", third);

// 	match v.get(20) {
// 		Some(third) => println!("The third element is {}", third),
// 		None => println!("There is no third element"),
// 	}
// }

// fn main() {
// 	let mut v = vec![100, 32, 57];
//     for i in &mut v {
// 		*i += 50;
// 	}

// 	for i in &v {
// 		println!("{}", i);
// 	}
// }

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
	
	for i in &row {
		println!("{:#?}", i);
	}
}
