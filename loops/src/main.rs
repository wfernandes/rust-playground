fn main() {
    let mut count = 0;

    //loop is an expression with a code block so technically the code block can have a return
    //value and that can be stored in a variable.
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };

    println!("result {}", result);
    println!("-------");
    countdown_loop();
    println!("-------");
    conditional_loop();
    println!("-------");
    another_countdown_loop();
}

fn countdown_loop() {
    let mut count = 3;

    while count != 0 {
        println!("{}!", count);
        count -= 1;
    }
    println!("liftoff!!");
}

fn another_countdown_loop() {
    // using for instead of while
    for i in (1..4).rev() {
        println!("{}!", i);
    }
    println!("booh yaka shah");
}

fn conditional_loop() {
    let a = [10, 20, 30, 40, 50, 60];

    for e in a.iter() {
        println!("value: {}", e);
    }
}
