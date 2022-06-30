enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let List = Cons(1, Cons(2, Cons(3, Nil)));
}
