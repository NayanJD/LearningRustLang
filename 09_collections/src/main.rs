use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    v.push(4);

    let second = v.get(1);

    match second {
        Some(second) => println!("Second element is {second}"),
        None => println!("There is no second element."),
    }

    let first = &v[0];

    // Line 21 would not allow this as line 19 does mutable borrow and line 21
    // does immutable borrow.
    // v.push(5);

    println!("The first element is {first}");

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i *= 2;
    }

    for i in &v {
        println!("{i}");
    }

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    // We are able to use s2 here because push_str takes string slice and
    // does not take ownership of s2.
    println!("s2: {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    // This is the function signature which is used here: fn add(self, s: &str) -> String.
    // Here self is s1 and s is s2. s1 would be moved here since it is a borrow in add function.
    let s3 = s1 + &s2;

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s3 = format!("{s4}-{s5}-{s6}");

    // String is a wrapper around Vec<u8>
    let s4 = String::from("hello");

    // Indexing of strings are not allowed.
    // let h = s4[0];

    let hello = String::from("Здравствуйте");

    // Using &hello[0..3] would crash. Use wisely!
    let s5 = &hello[0..4];

    println!("{s5}\n");

    // Do this instead
    for c in hello.chars() {
        println!("{c}");
    }

    println!();

    for c in hello.bytes() {
        println!("{c}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Red");
    let score = 60;

    // team_name and score are moved and not borrowedto the hash map
    scores.insert(team_name, score);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (team_name, score) in &scores {
        println!("{team_name}: {score}");
    }
} // When the vector v gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.
