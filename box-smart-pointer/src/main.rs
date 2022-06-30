enum List {
    Cons(i32, List),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);
}
