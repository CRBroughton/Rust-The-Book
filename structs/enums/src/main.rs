enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Here is a message")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    // let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // let x = 5;
    // let y: Option<i32> = None;

    // let sum = x + y.unwrap_or(0);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn route(ip_kind: IpAddrKind) {}


fn plus_one(x: Option<i32>) -> Option<i32> { // match expression
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}