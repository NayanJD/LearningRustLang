use addr::add;

// this can be run with cargo test --test integration_test
#[test]
fn it_adds_two() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
