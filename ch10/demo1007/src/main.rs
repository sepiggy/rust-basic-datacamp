struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some yearsa ago...");

    let first_sentence = novel.split('.').next().expect("Could not found a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
