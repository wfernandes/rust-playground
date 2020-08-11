#![allow(unused)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // We need this method to construct a Breakfast object. Since seasonal_fruit is private,
        // technically we cannot create an instance of Breakfast directly because we couldn't set
        // the value of seasonal_fruit on the Breakfast object.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // we only need the pub keyword on the enum and all of its variants become public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use with absolute path
// this is idiomatic for functions.
use crate::front_of_house::hosting;

// use with relattive path
// use self::front_of_house::hosting;

// when bringing in structs, enums and other items, its idiomatic to specify the full path
use std::collections::HashMap;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // call since we have 'use' bringing the path into scope.
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Summer time breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change mind about toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    // apps.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // specify the full path to a struct with use
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// exception to the above rule (regarding HashMap).
// if two items have the same name then regardless if they are structs, eumns, etc..
// we want to bring the items in this way.
use std::fmt;
// use std::io;

use std::io::Result as IOResult;

fn foobar1() -> fmt::Result {
    Ok(())
}
fn foobar2() -> IOResult<()> {
    Ok(())
}

// use nested paths to bring items to scope that are from similar crates
use std::{cmp::Ordering, io};
