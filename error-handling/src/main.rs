use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("There was a problem creating the file {:?}", err)
            }
            other_err => panic!("There was a problem opening the file {:?}", other_err)
        }
    };
}
