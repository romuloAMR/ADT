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
}