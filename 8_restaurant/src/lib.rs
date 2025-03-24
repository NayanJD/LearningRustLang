use std::io::{self, Write};

mod front_of_house;

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // This pubilc constructor is necessary because the struct breakfast has
        // one private member and this would not let outside code to create an
        // instance of it.
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("mango"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        // We are able to use deliver_order even if its not
        // public because children can access private parent
        // members.
        super::deliver_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod customer {

    // This is needed because hosting has been brought to scope
    // using `use` keyboard in the parent scope and not here.
    use crate::back_of_house;
    use crate::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();

        // super works in lieu of use crate::hosting;
        super::hosting::add_to_waitlist();

        let mut meal = back_of_house::Breakfast::summer("garlic");
        meal.toast = String::from("Rye");

        // This is not allowed since seasonal_fruit is private
        // meal.seasonal_fruit = String::from("grapes")

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}
