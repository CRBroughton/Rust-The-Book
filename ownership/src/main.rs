fn main() {
    // Mutable string example
    // println!("Hello, world!");

    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s);





    // Double free error example
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);





    // Clone example - data gets duplicated on the stack
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);





    // Function example with variables and out of scope behaviour
    // let s = String::from("hello");

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(x);



    // Returning values

    let s1 = gives_ownership(); // returns values

    let s2  = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s1);
    println!("{}", s3);

}


fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string // this is a returned value
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string //this is a returned valued
}


// fn takes_ownership(the_string: String) {
//     println!("{}", the_string);
// }

// fn makes_copy(the_int: i32) {
//     println!("{}", the_int);
// }