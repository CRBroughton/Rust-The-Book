fn main() {
    let number_list = vec![43, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The latest number is {}", largest);

    let char_list = vec!['y', 'z', 'a', 'b', 'c', 'd', 'e', 'f'];

    let largest = get_largest(char_list);

    println!("The latest number is {}", largest);
}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
