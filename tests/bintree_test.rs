use adt::models::bintree::BinTree;

#[test]
fn test_insert_empty() {
    let mut tree = BinTree::new();
    tree.insert(10);
    if let BinTree::Node(value, _, _) = tree {
        assert_eq!(value, 10);
    } else {
        panic!("Tree is not a node");
    }
}

#[test]
fn test_insert_left() {
    let mut tree = BinTree::new();
    tree.insert(10);
    tree.insert(5);
    if let BinTree::Node(value, left, right) = tree {
        assert_eq!(value, 10);
        if let BinTree::Node(left_value, _, _) = *left {
            assert_eq!(left_value, 5);
        } else {
            panic!("Left subtree is not a node");
        }
        assert!(matches!(*right, BinTree::Empty));
    } else {
        panic!("Tree is not a node");
    }
}

#[test]
fn test_insert_right() {
    let mut tree = BinTree::new();
    tree.insert(10);
    tree.insert(15);
    if let BinTree::Node(value, left, right) = tree {
        assert_eq!(value, 10);
        if let BinTree::Node(right_value, _, _) = *right {
            assert_eq!(right_value, 15);
        } else {
            panic!("Right subtree is not a node");
        }
        assert!(matches!(*left, BinTree::Empty));
    } else {
        panic!("Tree is not a node");
    }
}

#[test]
fn test_insert_duplicate() {
    let mut tree = BinTree::new();
    tree.insert(10);
    tree.insert(10);
    if let BinTree::Node(value, left, right) = tree {
        assert_eq!(value, 10);
        assert!(matches!(*left, BinTree::Empty));
        assert!(matches!(*right, BinTree::Empty));
    } else {
        panic!("Tree is not a node");
    }
}

#[test]
fn test_nodes() {
    let mut tree = BinTree::new();
    assert_eq!(tree.nodes(), 0);
    tree.insert(10);
    assert_eq!(tree.nodes(), 1);
    tree.insert(5);
    assert_eq!(tree.nodes(), 2);
    tree.insert(0);
    assert_eq!(tree.nodes(), 3);
    tree.insert(9);
    assert_eq!(tree.nodes(), 4);
    tree.insert(15);
    assert_eq!(tree.nodes(), 5);
    tree.insert(10);
    assert_eq!(tree.nodes(), 5);
    tree.insert(16);
    assert_eq!(tree.nodes(), 6);
}

#[test]
fn test_leaves() {
    let mut tree = BinTree::new();
    assert_eq!(tree.leaves(), 0);
    tree.insert(10);
    assert_eq!(tree.leaves(), 1);
    tree.insert(5);
    assert_eq!(tree.leaves(), 1);
    tree.insert(0);
    assert_eq!(tree.leaves(), 1);
    tree.insert(9);
    assert_eq!(tree.leaves(), 2);
    tree.insert(15);
    assert_eq!(tree.leaves(), 3);
    tree.insert(10);
    assert_eq!(tree.leaves(), 3);
    tree.insert(16);
    assert_eq!(tree.leaves(), 3);
}

#[test]
fn test_depth() {
    let mut tree = BinTree::new();
    assert_eq!(tree.depth(), 0);
    tree.insert(10);
    assert_eq!(tree.depth(), 1);
    tree.insert(5);
    assert_eq!(tree.depth(), 2);
    tree.insert(15);
    assert_eq!(tree.depth(), 2);
    tree.insert(20);
    assert_eq!(tree.depth(), 3);
}

#[test]
fn test_contains() {
    let empty_tree: BinTree<i32> = BinTree::Empty;
    let tree = BinTree::Node(
        10,
        Box::new(BinTree::Node(
            5,
            Box::new(BinTree::Empty),
            Box::new(BinTree::Empty),
        )),
        Box::new(BinTree::Node(
            15,
            Box::new(BinTree::Empty),
            Box::new(BinTree::Empty),
        )),
    );
    assert_eq!(empty_tree.contains(10), false);
    assert_eq!(tree.contains(10), true);
    assert_eq!(tree.contains(5), true);
    assert_eq!(tree.contains(15), true);
    assert_eq!(tree.contains(20), false);
}

#[test]
fn test_remove() {
    let mut tree = BinTree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.remove(5);
    assert_eq!(tree.contains(5), false);
    assert_eq!(tree.contains(3), true);
    assert_eq!(tree.contains(7), true);
    assert_eq!(tree.nodes(), 4);
    tree.remove(10);
    assert_eq!(tree.contains(10), false);
    assert_eq!(tree.contains(15), true);
    assert_eq!(tree.nodes(), 3);
}
