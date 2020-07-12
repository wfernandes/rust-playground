#![allow(unused)]

struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

// These are called Tuple Structs, they are essentially named tuples.
// Each one is it's own type, that is, although both of the types below have the same i32 values
// they cannot be interchanged. Also, just a tuple each individual value can be accessed via
// `.<index>`.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut u1 = build_user(String::from("example@email.com"), String::from("example"));
    let u2 = User {
        email: String::from("another@email.com"),
        username: String::from("another"),
        ..u1 // this is the struct update syntax which is used when we want to create another struct which has some fields with same values as u1.
    };
    println!("user: {}", u1.username);

    u1.email = String::from("foobar@email.com"); // In order for one of the struct fields to be
                                                 // mutated, the entire struct must be marked as mutable.

    let black = Color(0, 0, 0);
    let point = Point(1, 2, 3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // Shorthand notation since the function argument names have the exact same names as the struct fields.
        username,
        active: true,
        sign_in_count: 1,
    }
}
