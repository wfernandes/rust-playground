#![allow(unused)]

fn main() {
    out_of_bounds();
}

fn out_of_bounds() {
    let v = vec![1, 2, 3];
    v[99];
}

fn panic_now() {
    panic!("crash and burn");
}
