fn main() {
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let r = longest(string1.as_str(), string2);

        println!("The longest string is {}", r);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
