fn main() {
    // let mut s0 = "abcd";

    println!("Hello, world!");

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = s2;

    // println!("s2: {s2}"); // It will not work since s2 has already been moved.

    let mut s4 = String::from("A string");

    // s4.push_str(" another one");

    // change(&mut s4);

    let r1 = &s4;
    let r2 = &s4;
    // In line 23, Two mutable references not allowed since r1 and r2 are being used in
    // line 24. If the println! is not used in line 24, this line would have been
    // valid.
    // let r3 = &mut s4;
    println!("r1: {r1}, r2: {r2}, s4: {s4}");

    // IMP: Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    // and that's why we can create a mutable reference here since r2 and r1 are out of scope.
    let r3 = &mut s4;
    // let r4 = &mut s4;

    println!("r3: {r3}");

    let mut s5 = String::from("Hello world!");

    // type of s6 is &str
    let s6 = &s5[0..5];

    let word = first_word(s6);

    // s5.clear(); // This is not allowed since immutable borrow occurred in line 38 and 40

    println!("First word: {word}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn change(some_string: &mut String) {
    some_string.push_str(" is good.");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

// This function results in error and act as a reference
// fn dangle() -> &String {
//     // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
