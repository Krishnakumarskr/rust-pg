use data_structures::stack::Stack;

#[test]
fn test_stac_fn() {
    let mut stack = Stack::new();

    stack.push(32);
    stack.push(45);

    assert_eq!(stack.length(), 2);
    assert_eq!(stack.pop(), Some(45));
    assert_eq!((stack.length()), 1);
}









