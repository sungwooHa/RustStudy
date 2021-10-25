extern crate integration_testing;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_testing::add_two(2));
}