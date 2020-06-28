fn main() {
    let n = 3;

    if n < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // expressions in both code blocks must evaluate to the same type
    // since if is an expression it can evaluate a result and store them into a variable.
    let n = if true { 88 } else { 99 };
    println!("n: {}", n);
}
