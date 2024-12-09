use common::util::traversal;

#[test]
pub fn root_parentheses_traversal_works() {
    let input = "(hello: (), world: Fn() -> String)";

    let idx = traversal::traverse_till_root_par(input.as_bytes(), (b'(', b')'));

    assert_eq!(idx, Some(33))
}
