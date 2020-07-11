#![allow(unused)]
fn main() {
    let s = String::from("hello world");

    let fw = first_word(&s);
    println!("{}", fw);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &it) in bytes.iter().enumerate() {
        if it == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
