use adt::models::deque::Deque;

#[test]
fn test_push_front() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_front(1);
    deque.push_front(2);
    assert_eq!(deque.peek_front(), Some(&2));
    assert_eq!(deque.peek_back(), Some(&1));
}

#[test]
fn test_push_back() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(1);
    deque.push_back(2);
    assert_eq!(deque.peek_front(), Some(&1));
    assert_eq!(deque.peek_back(), Some(&2));
}

#[test]
fn test_pop_front() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(1);
    deque.push_back(2);
    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_front(), Some(2));
    assert_eq!(deque.pop_front(), None);
}

#[test]
fn test_pop_back() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(1);
    deque.push_back(2);
    assert_eq!(deque.pop_back(), Some(2));
    assert_eq!(deque.pop_back(), Some(1));
    assert_eq!(deque.pop_back(), None);
}

#[test]
fn test_peek_front() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(1);
    assert_eq!(deque.peek_front(), Some(&1));
    deque.push_back(2);
    assert_eq!(deque.peek_front(), Some(&1));
    deque.push_front(3);
    assert_eq!(deque.peek_front(), Some(&3));
}

#[test]
fn test_peek_back() {
    let mut deque: Deque<i32> = Deque::new();
    deque.push_back(1);
    assert_eq!(deque.peek_back(), Some(&1));
    deque.push_back(2);
    assert_eq!(deque.peek_back(), Some(&2));
    deque.push_front(3);
    assert_eq!(deque.peek_back(), Some(&2));
}

#[test]
fn test_is_empty() {
    let mut deque: Deque<i32> = Deque::new();
    assert!(deque.is_empty());
    deque.push_back(1);
    assert!(!deque.is_empty());
}

#[test]
fn test_size() {
    let mut deque: Deque<i32> = Deque::new();
    assert_eq!(deque.size(), 0);
    deque.push_back(1);
    assert_eq!(deque.size(), 1);
    deque.push_front(2);
    assert_eq!(deque.size(), 2);
}
