fn main() {
    let x  = 5;
    let y = Box::new(x); // Reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // deref
}
