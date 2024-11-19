use adt::models::list::List;

#[test]
fn test_new() {
    let list: List<i32> = List::new();
    assert_eq!(list, List::Empty);
}

#[test]
fn test_is_empty() {
    let list: List<i32> = List::new();
    assert!(list.is_empty());
}

#[test]
fn test_size() {
    let list: List<i32> = List::new();
    assert_eq!(list.size(), 0);
    let list = list.push_front(1);
    assert_eq!(list.size(), 1);
    let list = list.push_front(3);
    assert_eq!(list.size(), 2);
}

#[test]
fn test_push_front() {
    let mut list = List::new().push_front(1);
    assert_eq!(list, List::Cons(1, Box::new(List::Empty)));
    list = list.push_front(2).push_front(3);
    assert_eq!(
        list,
        List::Cons(
            3,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(1, Box::new(List::Empty)))
            ))
        ),
    )
}

#[test]
fn test_pop_front() {
    let list = List::new().push_front(1).push_front(2);
    let list = list.pop_front();
    assert_eq!(list, List::Cons(1, Box::new(List::Empty)));
}

#[test]
fn test_pop_back() {
    let list = List::new().push_front(1).push_front(2).push_front(3);
    let list = list.pop_back();
    assert_eq!(
        list,
        List::Cons(3, Box::new(List::Cons(2, Box::new(List::Empty))))
    );
}

#[test]
fn test_insert() {
    let list = List::new().push_front(1).push_front(3);
    let list = list.insert(1, 2);
    assert_eq!(
        list,
        List::Cons(
            3,
            Box::new(List::Cons(
                2,
                Box::new(List::Cons(1, Box::new(List::Empty)))
            ))
        )
    );
}

#[test]
fn test_remove() {
    let list = List::new().push_front(1).push_front(2).push_front(3);
    let list = list.remove(1);
    assert_eq!(
        list,
        List::Cons(3, Box::new(List::Cons(1, Box::new(List::Empty))))
    );
}

#[test]
fn test_get() {
    let list = List::new().push_front(1).push_front(2).push_front(3);
    assert_eq!(list.get(0), Some(&3));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&1));
    assert_eq!(list.get(3), None);
}
