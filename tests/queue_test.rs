use adt::models::queue::Queue;

#[test]
fn test_new() {
    let queue: Queue<i32> = Queue::new();
    assert!(queue.is_empty());
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_push() {
    let mut queue = Queue::new();
    queue.push(1);
    assert!(!queue.is_empty());
    queue.push(2);
    assert_eq!(queue.size(), 2);
    assert_eq!(*queue.peek().unwrap(), 1);
}

#[test]
fn test_pop() {
    let mut queue = Queue::new();
    queue.push(1);
    queue.push(2);
    let popped_value = queue.pop();
    assert_eq!(popped_value, Some(1));
    assert_eq!(queue.size(), 1);
    assert_eq!(*queue.peek().unwrap(), 2);
}

#[test]
fn test_peek() {
    let mut queue = Queue::new();
    assert!(queue.peek().is_none());
    queue.push(1);
    assert_eq!(*queue.peek().unwrap(), 1);
}

#[test]
fn test_is_empty() {
    let mut queue = Queue::new();
    assert!(queue.is_empty());
    queue.push(1);
    assert!(!queue.is_empty());
    queue.pop();
    assert!(queue.is_empty());
}

#[test]
fn test_size() {
    let mut queue = Queue::new();
    assert_eq!(queue.size(), 0);
    queue.push(1);
    assert_eq!(queue.size(), 1);
    queue.push(2);
    assert_eq!(queue.size(), 2);
    queue.pop();
    assert_eq!(queue.size(), 1);
}
