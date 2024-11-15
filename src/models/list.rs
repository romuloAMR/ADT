#[derive(Debug, PartialEq)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Empty
    }

    pub fn is_empty(&self) -> bool {
        match self {
            List::Empty => true,
            _ => false,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Cons(_, tail) => 1 + tail.size(),
        }
    }

    pub fn push_front(self, value: T) -> List<T> {
        List::Cons(value, Box::new(self))
    }

    pub fn pop_front(self) -> List<T> {
        match self {
            List::Empty => List::Empty,
            List::Cons(_, tail) => *tail,
        }
    }

    pub fn pop_back(self) -> List<T> {
        match self {
            List::Empty => List::Empty,
            List::Cons(_, ref tail) if tail.is_empty() => List::Empty,
            List::Cons(head, tail) => List::Cons(head, Box::new(tail.pop_back())),
        }
    }

    pub fn insert(self, index: usize, value: T) -> List<T> {
        match (index, self) {
            (0, list) => list.push_front(value),
            (_, List::Empty) => List::Empty,
            (i, List::Cons(head, tail)) => List::Cons(head, Box::new(tail.insert(i - 1, value))),
        }
    }

    pub fn remove(self, index: usize) -> List<T> {
        match (index, self) {
            (_, List::Empty) => List::Empty,
            (0, List::Cons(_, tail)) => *tail,
            (i, List::Cons(head, tail)) => List::Cons(head, Box::new(tail.remove(i - 1))),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match (index, self) {
            (_, List::Empty) => None,
            (0, List::Cons(head, _)) => Some(head),
            (i, List::Cons(_, tail)) => tail.get(i - 1),
        }
    }
}