#[derive(Debug, PartialEq)]
pub enum BinTree<T> {
    Empty,
    Node(T, Box<BinTree<T>>, Box<BinTree<T>>),
}

impl<T: Ord + Copy> BinTree<T> {
    // Creates a new empty binary tree
    pub fn new() -> Self {
        BinTree::Empty
    }

    // Inserts a value into the binary tree
    pub fn insert(&mut self, value: T) {
        match self {
            BinTree::Empty => {
                *self = BinTree::Node(value, Box::new(BinTree::Empty), Box::new(BinTree::Empty))
            }
            BinTree::Node(data, ref mut left, ref mut right) => {
                if value < *data {
                    left.insert(value);
                } else if value > *data {
                    right.insert(value);
                }
            }
        }
    }

    // Counts the total number of nodes in the tree
    pub fn nodes(&self) -> usize {
        match self {
            BinTree::Empty => 0,
            BinTree::Node(_, m, n) => 1 + m.nodes() + n.nodes(),
        }
    }

    // Counts the number of leaf nodes in the tree
    pub fn leaves(&self) -> usize {
        match self {
            BinTree::Empty => 0,
            BinTree::Node(_, ref m, ref n) => match (m.as_ref(), n.as_ref()) {
                (BinTree::Empty, BinTree::Empty) => 1,
                (_, _) => m.leaves() + n.leaves(),
            },
        }
    }

    // Calculates the depth of the tree
    pub fn depth(&self) -> usize {
        match self {
            BinTree::Empty => 0,
            BinTree::Node(_, ref m, ref n) => {
                let m_depth: usize = 1 + m.depth();
                let n_depth: usize = 1 + n.depth();
                if m_depth < n_depth {
                    n_depth
                } else {
                    m_depth
                }
            }
        }
    }

    // Checks if the tree contains a specific value
    pub fn contains(&self, value: T) -> bool {
        match self {
            BinTree::Empty => false,
            BinTree::Node(data, ref left, ref right) => {
                if value == *data {
                    return true;
                }
                if value < *data {
                    left.contains(value)
                } else {
                    right.contains(value)
                }
            }
        }
    }

    // Removes a value from the binary tree
    pub fn remove(&mut self, value: T) {
        match self {
            BinTree::Empty => return,
            BinTree::Node(data, ref mut left, ref mut right) => {
                if value < *data {
                    left.remove(value);
                } else if value > *data {
                    right.remove(value);
                } else {
                    if left.is_empty() && right.is_empty() {
                        *self = BinTree::Empty;
                    } else if left.is_empty() {
                        *self = std::mem::replace(right, BinTree::Empty);
                    } else if right.is_empty() {
                        *self = std::mem::replace(left, BinTree::Empty);
                    } else {
                        let min_value = right.min_value().unwrap();
                        *data = min_value;
                        right.remove(min_value);
                    }
                }
            }
        }
    }

    // Finds the minimum value in the tree
    fn min_value(&self) -> Option<T>
    where
        T: Copy,
    {
        match self {
            BinTree::Empty => None,
            BinTree::Node(data, ref left, _) => {
                if let BinTree::Empty = **left {
                    Some(*data)
                } else {
                    left.min_value()
                }
            }
        }
    }

    // Checks if the tree is empty
    fn is_empty(&self) -> bool {
        matches!(self, BinTree::Empty)
    }
}
