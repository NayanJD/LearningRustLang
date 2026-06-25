use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let vegetable = Asparagus {};

    println!("I'm growing vegetable {vegetable:?}!");
}
