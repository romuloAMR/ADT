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