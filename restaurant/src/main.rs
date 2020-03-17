/*
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            println!("add a customer to wait list.");
        }

        fn seat_at_table() {
            println!("seat at a table, wait for the food to come.");
        }
    }

    mod serving {
        fn take_order() {
            println!("take an order of a customer.")
        }

        fn serve_order() {
            println!("serve an order of a customer.")
        }

        fn take_payment() {
            println!("the customer finish the food, the pay for it!");
        }
    }
}


mod top_mod {
    pub mod inner_mod {
        pub fn test() {
            println!("This is a test function!");
        }
    }
}

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
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to wait list");
        }
    }
}

use std::collections::HashMap;
 */
mod jet_fighter;

use crate::jet_fighter::suhoyi;

fn main() {
    suhoyi::show_suhoyi_fighters();
}
