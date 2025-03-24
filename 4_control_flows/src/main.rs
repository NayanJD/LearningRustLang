fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 2 * counter;
        }
    };

    println!("Result: {result}");

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("{element}");
    }

    for element in (1..=4).rev() {
        println!("{element}");
    }

    println!("LIFT OFF!");
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
