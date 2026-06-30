use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();

    v.push(5);

    // This does not work because the first value inserted to v was an i32
    // v.push(3.0);

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    let first = &v[0];

    // This does a mutable borrow
    v.push(6);

    // This line would not compile because of borrowing rules
    // println!("The first element is {first}");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    {
        let v = vec![1, 2, 3, 4, 5];
    } // <- v goes out of scope and is freed here

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    // s2 still works because push_str takes a referecne (&str)
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // s1 has been moved here (changed ownership) and is not valid hereafter

    // This will not work
    // println!("s1: {s1}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Because of this behavior of + (add) operator it's better to use format!
    // let s = s1 + "-" + &s2 + "-" + &s3;

    let s = format!("{s1}-{s2}-{s3}");

    // This is in UTF-8 with 4 bytes of memory stored as
    // Vec<u8>
    let hello = String::from("Hola");

    // This is in UTF-8 with 24 bytes of memory
    let hello = String::from("Здравствуйте");

    // This is not allowed by rust. One of the reason Rust doesn’t allow us to
    // index into a String to get a character is that indexing operations are
    // expected to always take constant time (O(1)). But it isn’t possible to
    // guarantee that performance with a String, because Rust would have to walk
    // through the contents from the beginning to the index to determine how
    // many valid characters there were.
    // let answer = &hello[0];

    // This might also crash depending upon what value for index we provide
    let answer = &hello[0..4];

    println!("Printing bytes for नमस्ते");

    for b in "नमस्ते".bytes() {
        print!("{b} ");
    }
    println!();

    println!("Printing scalar values for नमस्ते");

    for c in "नमस्ते".chars() {
        print!("{c} ");
    }

    println!();

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    let team_name = String::from("blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} => {value}");
    }

    let field_name = String::from("Favorite number");
    let field_value = 35;

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // field_name is String and it was moved to HashMap map. Hence, it
    // can't be used here. field_value is i32 and implements Copy trait.
    // So, it can be used here.
    // println!("{field_name} => {field_value}");

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);

    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    println!("{scores:?}");

    let mut count_map = HashMap::new();

    let text = "hello world how are you world";

    for word in text.split_whitespace() {
        let count = count_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("word count: {count_map:?}");
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
