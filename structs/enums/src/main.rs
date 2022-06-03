enum IpAddrKind {
    V4(String),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));
}

fn rout(ip_kind: IpAddrKind) {}