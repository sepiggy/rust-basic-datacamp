//region 函数的基本用法
// fn main() {
//     println!("Hello, world!");
//     another_function1();
//     another_function2(100,200); // argument
// }
//
// fn another_function1() {
//     println!("Another function");
// }
//
// fn another_function2(x: i32, y: i32) { // parameter
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }
//endregion

//region Rust是面向表达式的语言
// fn main() {
//     let x = 5;
//     let y = {  // 这个大括号就是表达式
//         let x = 1;
//         x + 3
//     };
//
//     println!("The value of y is: {}", y);
// }
//endregion

//region 函数的返回值
// 以函数体的最后一个表达式作为返回值
// fn main() {
//     fn plus_five(x: i32) -> i32 {
//         5 + x
//     }
//
//     let x = plus_five(100);
//
//     println!("The value of x is: {}", x)
// }
//endregion