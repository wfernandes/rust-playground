// these are crates
// Rng is a trait
use rand::Rng;
// Ordering is an enum
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // the ! makes this a macro and not a function call
        println!("Please input your guess: ");

        // new is an associated function of the String type. An associated function is implemented
        // on a type. Similar to static functions.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing the variable guess - allows for converting between types and reuse of the
        // variable.
        // match expressions are made up of "arms". An "arm" consists of a pattern and code.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                // {} is like %s
                println!("err: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
