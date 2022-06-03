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

    let x = 5;
    let y: Option<i32> = None;

    let sum = x + y.unwrap_or(0);
}

fn route(ip_kind: IpAddrKind) {}