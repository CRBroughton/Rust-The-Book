enum List {
    Const(i32, List),
    Nil,
}

use List::{Const, Nil};

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);
}
