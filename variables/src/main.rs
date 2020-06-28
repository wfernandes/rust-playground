fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // Shadowing works when we use 'let'.
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Shadowing
    // We need to use 'let' in order to do transformations.
    // Simply assigning a variable to a value will result in a compile time error.
    // The other difference between Shadowing and mut is that because we are using the let keyword
    // we are technically creating a new variable and hence can change the type of the variable.

    let spaces = "    ";
    let spaces = spaces.len();
    println!("number of spaces: {}", spaces);
}
