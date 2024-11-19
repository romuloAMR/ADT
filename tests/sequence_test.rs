use adt::models::sequence::Sequence;

#[test]
fn test_new_sequence() {
    let seq: Sequence<i32> = Sequence::new();
    assert_eq!(seq.size(), 0);
}

#[test]
fn test_add() {
    let mut seq: Sequence<i32> = Sequence::new();
    seq.add(1);
    assert_eq!(seq.size(), 1);
    assert_eq!(seq.get(0), Some(&1));
}

#[test]
fn test_remove() {
    let mut seq: Sequence<i32> = Sequence::new();
    seq.add(10);
    let removed = seq.remove();
    assert_eq!(removed, Some(10));
    assert_eq!(seq.size(), 0);
}

#[test]
fn test_resize_up() {
    let mut seq: Sequence<i32> = Sequence::new();
    for i in 0..9 {
        seq.add(i);
    }
    assert_eq!(seq.size(), 9);
}

#[test]
fn test_resize_down() {
    let mut seq: Sequence<i32> = Sequence::new();
    for i in 0..16 {
        seq.add(i);
    }
    for _ in 0..9 {
        seq.remove();
    }
    assert_eq!(seq.size(), 7);
}

#[test]
fn test_get() {
    let mut seq: Sequence<i32> = Sequence::new();
    seq.add(20);
    assert_eq!(seq.get(0), Some(&20));
    assert_eq!(seq.get(1), None);
}
