use test01;

mod common;

// cargo test -- --show-output
#[test]
fn test_add() {
    common::setup();
    assert_eq!(3, test01::add(1, 2));
}