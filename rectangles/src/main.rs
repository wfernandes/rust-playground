#![allow(unused)]
#[derive(Debug)] // Can be used to "debug" structs using println!

// structs don't implement the Display trait since there can be many possibilities.
struct Rect {
    width: u32,
    height: u32,
}

// each struct can have multiple impl blocks if necessary
impl Rect {
    // create a method for Rect
    fn area(&self) -> u32 {
        // we still want to borrow self immutably.
        self.width * self.height
    }

    // this is still a method, because it takes in an instance of the struct
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is an associated function since it doesn't work on the struct itself.
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn area(r: &Rect) -> u32 {
    r.width * r.height
}

fn main() {
    let r = Rect {
        width: 4,
        height: 10,
    };

    let r2 = Rect {
        width: 5,
        height: 9,
    };

    println!("Rect: {:#?}", r); // Other option for single line format struct is {:?}
    println!("Area of rect r: {}", r.area());
    println!("Area of rect r2: {}", area(&r2));

    println!("Can r hold r2? {}", r.can_hold(&r2));

    let sq = Rect::square(7); // This function is namespaced by the struct: the :: syntax is used
                              // for both associated functions and namespaces created by modules
    println!("Area of sq: {}", sq.area());
}
