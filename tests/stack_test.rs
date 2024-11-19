use adt::models::stack::Stack;

#[test]
fn test_new() {
    let stack: Stack<i32> = Stack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_push() {
    let mut stack = Stack::new();
    stack.push(1);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
    assert_eq!(*stack.peek().unwrap(), 1);
}

#[test]
fn test_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    let popped_value = stack.pop();
    assert_eq!(popped_value, Some(2));
    assert_eq!(stack.size(), 1);
    assert_eq!(*stack.peek().unwrap(), 1);
}

#[test]
fn test_peek() {
    let mut stack = Stack::new();
    assert!(stack.peek().is_none());
    stack.push(1);
    assert_eq!(*stack.peek().unwrap(), 1);
}

#[test]
fn test_is_empty() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());
    stack.push(1);
    assert!(!stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
}

#[test]
fn test_size() {
    let mut stack = Stack::new();
    assert_eq!(stack.size(), 0);
    stack.push(1);
    assert_eq!(stack.size(), 1);
    stack.push(2);
    assert_eq!(stack.size(), 2);
    stack.pop();
    assert_eq!(stack.size(), 1);
}
