pub trait Messenger {
    fn send(&self, msg: &str);
}

fn main() {
    let x = 5;
    let y = &mut x;
}
