pub enum BinTree<T> {
    Empty,
    Node(T, Box<BinTree<T>>, Box<BinTree<T>>)
}

impl<T: Ord> BinTree<T> {
    pub fn new () -> Self{
        BinTree::Empty
    }
    pub fn insert (&mut self, value: T){
        match self {
            BinTree::Empty => *self = BinTree::Node(value, Box::new(BinTree::Empty), Box::new(BinTree::Empty)),
            BinTree::Node(
                data,
                ref mut left,
                ref mut right
            ) => {
                    if value < *data {
                        left.insert(value);
                    } else if value > *data {
                        right.insert(value);
                    } else {
                        return
                    }
                }
        }
    }
    pub fn nodes(&self) -> usize{
        match self {
            BinTree::Empty => 0,
            BinTree::Node(_, m, n) => 1 + m.nodes() + n.nodes()
        }
    }
    pub fn leaves(&self) -> usize{
        match self {
            BinTree::Empty => 0,
            BinTree::Node(_, ref m, ref n) => {
                match (m.as_ref(), n.as_ref()) {
                    (BinTree::Empty, BinTree::Empty) => 1,
                    (_,_) => m.leaves() + n.leaves()
                }
            }
        }
    }
    pub fn depth(&self) -> usize{
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
}