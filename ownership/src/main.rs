fn main() {
    // Mutable string example
    // println!("Hello, world!");

    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s)


    // Double free error example
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // Clone example - data gets duplicated on the stack
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
