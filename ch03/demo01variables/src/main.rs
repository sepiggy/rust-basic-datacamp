const MAX_POINTS: i32 = 100_000;

//region mut
// fn main() {
//     let mut x = 5;
//     println!("The value of x is {}", x);
//     println!("The max points is {}", MAX_POINTS);
//     x = 100;
// }
//endregion

//region shadowing
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is {}", x);
//     let spaces = "    ";
//     let spaces = spaces.len();
//     println!("The value of spaces is {}", spaces);
// }
//endregion

//region 类型标注
// fn main() {
//     let guess = "42".parse().expect("Not a number");
//     println!("{}", guess);
// }
//endregion

//region 标量类型
// fn main() {
//     let x = 2.0;
//     let y: f32 = 3.0;
// }
//endregion

//region Tuple
// fn main() {
//     let tup = (500, 6.4, 1);
//     // 访问Tuple中的元素
//     println!("{}, {}, {}", tup.0, tup.1, tup.2);
//
//     // Tuple的解构
//     let (x, y, z) = tup;
//     println!("{}, {}, {}", x, y, z);
// }
//endregion

//region 数组
fn main() {
    // 数组存储在栈上
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("{}", a[4]);

}
//endregion
