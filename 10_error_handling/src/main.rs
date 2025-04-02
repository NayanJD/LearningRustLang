use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn!");

    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    // Basic error handling
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Found error while opening hello.txt: {error:?}"),
    // };

    // Error handling along with file creation if the file is not found
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Couldn't create file because of error: {error}"),
    //         },
    //         other_error => panic!("Problem opening the file due to unknown reason: {other_error}"),
    //     },
    // };

    // unwrap() un-wraps the file value from Ok(file) from previous match case and panics if there
    // is an error
    let greeting_file = File::open("hello.txt").unwrap();

    // expect() works the same as unwrap() with the exception that we can provide the &str which
    // panic should use.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // We cannot do this if the main() has no return type which () in this case. ? is compatible
    // with return types which implement FromResidual trait as shown in the error
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// This function demonstract error propagation
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(error) => Err(error),
//     }
// }

// This example uses ? to return the error. ? operator uses "from" trait to convert errors from
// source to the type which the function returns (return type declaration).
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//
//     let mut username = String::new();
//
//     username_file.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// ? can also be used in chaining as in ?.
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// read_to_string() from the standard library provides a handy util to do what we were trying to do
// till now
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Example of how to do ?. chaining with Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
