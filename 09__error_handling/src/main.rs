use std::error;
use std::io::{self, Read};
use std::{fs::File, io::Error, io::ErrorKind};

fn main() -> Result<(), Box<dyn error::Error>> {
    // panic!("crash and burn!");

    let v = vec![1, 2, 3];

    // this will panic. Run with RUST_BACKTRACE=1
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Could not create file: {error:?}"),
            },
            _ => panic!("file could not be opened because of uknown error: {error:?}"),
        },
    };

    // let greeting_file = File::open("world.txt").unwrap();

    let greeting_file =
        File::open("world.txt").expect("world.txt should be included in this project");

    let greeting_file = File::open("hello.txt");

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    // ok method of Result converts it to Option<result>. Just FYI
    // username_file_result.ok();

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Result::Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_with_operator() -> Result<String, io::Error> {
    // ? operator returns the result value of Ok(result) on success.
    // For Err(error), it would be returned as is or from method is called if
    // From trait implemented
    let mut username_file = File::open("username.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!("value is not between 1-100");
        }

        Guess { value: val }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
