// This will import std::io and std::io::Write into scope
use std::io::{self, Write};

// Importing all public types
use std::collections::*;

mod front_of_house;

pub use crate::front_of_house::hosting;

// This does not work. This has to be in src/main.rs
// pub mod garden;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Using "use"
    hosting::add_to_waitlist();

    // This will fail because seasonal_fruit is private
    // let breakfast = back_of_house::Breakfast {
    //     toast: String::from("rye"),
    // };

    let mut meal = back_of_house::Breakfast::summer("rye");

    meal.toast = String::from("Wheat");

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let appetizer1 = back_of_house::Appetizer::Soup;
    let appetizer2 = back_of_house::Appetizer::Salad;
}

mod customer {
    // hosting need to be brought into this scope for eat_at_restaurant
    // to use it or use super
    // use crate::front_of_house::hosting;

    fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        // We think the back_of_house module and the deliver_order
        // function are likely to stay in the same relationship to
        // each other and get moved together should we decide to
        // reorganize the crate’s module tree. Therefore, we used
        // super so that we’ll have fewer places to update code in
        // the future if this code gets moved to a different module.
        super::deliver_order();
    }

    fn cook_order() {}
}
