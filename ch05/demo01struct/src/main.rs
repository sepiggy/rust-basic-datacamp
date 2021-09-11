struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct

fn main() {
    let mut user = User {
        username: (String::from("abc@126.com")),
        email: (String::from("Nikky")),
        sign_in_count: (556),
        active: (true),
    };

    user.email = String::from("anotheremail@example.com");
    let user2 = build_user(String::from("xxx@email.com"), String::from("xxx"));
    
    let user3 = User {
        email: String::from("jms@sina.com"),
        username: String::from("jms"),
        ..user2
    };
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}
