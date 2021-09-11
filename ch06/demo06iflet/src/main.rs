fn main() {
    let v = Some(3u8);
    let v = Some(5u8);

    if let Some(3) = v {
        println!("three");
        println!("{:#?}", v);
    } else {
        println!("others");
        println!("{:#?}", v);
    }
}
