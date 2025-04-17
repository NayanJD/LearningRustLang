enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self) -> u32 {
        match self {
            UsState::Alabama => 1981,
            UsState::Alaska => 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, world!");

    let some_char = Some('c');
    let some_number = Some(5);

    let absent_number: Option<i32> = None;

    let x: i32 = 1;
    let y: Option<i32> = None;

    // This is not allowed. When we have a value of a type like i8 in Rust,
    // the compiler will ensure that we always have a valid value. We can
    // proceed confidently without having to check for null before using that value.
    // let z = x + y;
    //

    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value of a quarter in cent is {cents}");

    let five = plus_one(Some(4));
    let none = plus_one(None);

    let mut count = 0;

    let coin = Coin::Quarter(UsState::Alabama);

    // match coin {
    //     Coin::Quarter(state) => println!("We got a quarter from {state:?} state."),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("We got a quarter from {state:?} state.");
    } else {
        count += 1;
    }

    let coin = Coin::Quarter(UsState::Alaska);

    if let msg = describe_state(coin) {
        println!("Here is the message about the coin: {msg:?}")
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Look we got a quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in() < 1900 {
        Some(format!("The state {state:?} is old!"))
    } else {
        Some(format!("The state {state:?} is new!"))
    }
}
