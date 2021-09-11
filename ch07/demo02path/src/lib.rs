mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 关联函数
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("{:#?}", meal);
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("{:#?}", meal);
    // meal.seasonal_fruit = String::from("blueberries");
}
