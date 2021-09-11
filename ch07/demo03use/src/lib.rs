// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
// 

mod front_of_house;

// 将模块引入到当前作用域
// use crate::front_of_house::hosting;
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult;
