#[cfg(test)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width == other.width && self.height == other.height
    }
}

mod tests {
    use core::panic;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn failing_test() {
        panic!("Failing test");
    }
}
