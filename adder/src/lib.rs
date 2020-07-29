#![allow(unused)]
#[derive(Debug)] // Can be used to "debug" structs using println!

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    pub fn foo(value: i32) -> Guess {
        if value == 3 {
            panic!("the value is 3");
        } else if value == 4 {
            panic!("the value is 4");
        }
        Guess { value }
    }
}

fn main() {}

// We are telling Rust that this module has a configuration with an option. And the option is
// 'test'.
#[cfg(test)]
mod tests {
    // The test module is like any other module and hence follows usual visibility rules. Since it
    // is an inner module, we need to bring the outer code into scope so it can be tested.
    use super::*;

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_test() {
        // panic!("failed test");
    }

    #[test]
    fn it_adds_two() {
        // Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=,
        // respectively. When the assertions fail, these macros print their arguments using debug
        // formatting, which means the values being compared must implement the PartialEq and Debug
        // traits. All the primitive types and most of the standard library types implement these
        // traits.
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    // Since we are expecting the test to panic, we annotate it with that expectation.
    #[test]
    #[should_panic]
    fn guess_greater_than_100() {
        Guess::new(101);
    }

    #[test]
    // expected is the substring of the panic message
    #[should_panic(expected = "value is 3")]
    fn guess_foo() {
        Guess::foo(3);
    }

    // Write a test that returns a Result.
    // We cannot use #[should_panic] on tests that return a Result.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 does not equal 4"))
        }
    }
}
