fn main() {
    println!("Hello, world!");
    another_function(5, 45);

    let xx = {
        let x = 4;
        x * 2
    };
    println!("xx: {}", xx);

    // treat multiple return values as a tuple
    let (xp, xpp) = plus_one(5);
    println!("xp, xpp: {} {}", xp, xpp);
}

fn plus_one(x: i32) -> (i32, i32) {
    // multiple return values are essentially tuples
    (x + 1, x + 2)
}

fn another_function(x: i32, y: i32) {
    // the argument x needs to be used before being shadowed else the compiler will throw an error
    println!("x: {}", x);
    let x = "foo";
    println!("x: {}", x);
    println!("y: {}", y);
}
