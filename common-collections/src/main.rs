fn main() {
    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];
    let third = &v2[2];
    // println!("'The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("This is no third element")
    }

    for i in &v {
        println!("{}", i);
    }
}
