use adder;

mod common;

// Since this test is in the tests/ directory, we don't need the #[cfg(test)]
#[test]
fn integration_it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
