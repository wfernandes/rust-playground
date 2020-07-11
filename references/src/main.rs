#![allow(unused)]
fn main() {
    // Exercise 1
    let s1 = String::from("hello world");

    let len = calc_len(&s1);

    println!("len of '{}' is {}", s1, len);

    // Exercise 2
    let mut s = String::from("hello");

    // Since we cannot have a immutable and mutable references simultaneously we need to scope them
    // appropriately. We could technically put the next three lines within a separate scope {}.
    let s1 = &s;
    let s2 = &s;
    println!("{}, {}, ", s1, s2); // This is SUPER COOL!! Since the last usage of s1 and s2 is over
                                  // here, the scope of s1 and s2 ends here. If the println was
                                  // moved to the end of the fn, it would be considered an error.
    let s3 = &mut s;

    // Exercise 3 - Dangling References
    let reference_to_nothing = dangle();
}

/// calc_len returns the length of the string passed.
fn calc_len(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope here. However, because it does not have ownership of anything nothing
  // happens here.

fn dangle() -> &String {
    let s = String::from("foobar");
    &s // we return a reference to the String s
} // Here, s goes out of scope and the memory that is pointing to is dropped. So in order to return
  // this string, we should just give ownership of it back to the caller. See Ownership example.
