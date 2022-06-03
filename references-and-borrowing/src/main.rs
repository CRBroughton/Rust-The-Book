// fn main() {
//     let s1 = String::from("hello world");
//     let len = calculate_length(&s1); // Similar to C, passes without giving up ownership

//     println!("The length of '{}' is {}.", s1, len);
// }


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// Mutable references

fn main() {
    let mut s = String::from("hello world");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}