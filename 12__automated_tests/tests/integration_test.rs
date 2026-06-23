use addr::add;

mod common;

// this can be run with cargo test --test integration_test
#[test]
fn it_adds_two() {
    common::setup();

    let result = add(2, 2);
    assert_eq!(result, 4);
}
