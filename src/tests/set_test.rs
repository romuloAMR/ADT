use adt::models::set::Set;

#[test]
fn test_new_set() {
    let set: Set<i32> = Set::new();
    assert_eq!(set.array.len(), 0);
}

#[test]
fn test_add() {
    let mut set = Set::new();
    set.add(1);
    assert!(set.contains(&1));
    assert_eq!(set.array.len(), 1);

    set.add(1); // Adicionar novamente não deve alterar o conjunto
    assert_eq!(set.array.len(), 1);
}

#[test]
fn test_remove() {
    let mut set = Set::new();
    set.add(1);
    set.add(2);

    assert!(set.remove(&1));
    assert!(!set.contains(&1));
    assert!(set.contains(&2));
    assert_eq!(set.array.len(), 1);

    assert!(!set.remove(&3)); // Remover um valor que não está no conjunto
}

#[test]
fn test_contains() {
    let mut set = Set::new();
    set.add(1);
    assert!(set.contains(&1));
    assert!(!set.contains(&2));
}
