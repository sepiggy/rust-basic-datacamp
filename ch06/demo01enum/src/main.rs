#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrKind1::V4(127, 0, 0, 1);
    let loopback = IpAddrKind1::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}
