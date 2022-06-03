fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s)
}
