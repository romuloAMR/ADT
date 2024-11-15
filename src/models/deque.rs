pub struct Deque<T> {
    data: Vec<T>
}

impl<T> Deque<T> {
    // Create a new deque
    pub fn new() -> Self {
        Deque {
            data: Vec::new()
        }
    }

    // Insert element at the start
    pub fn push_front(&mut self, value: T){
        self.data.insert(0, value);
    }

    // Insert element at the end
    pub fn push_back(&mut self, value: T){
        self.data.push(value);
    }

    // Remove element at the start
    pub fn pop_front(&mut self) -> Option<T>{
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    // Remove element at the end
    pub fn pop_back(&mut self) -> Option<T>{
        self.data.pop()
    }

    // Show first element
    pub fn peek_front(&self) -> Option<&T> {
        self.data.first()
    }

    // Show last element
    pub fn peek_back(&self) -> Option<&T> {
        self.data.last()
    }

    // Check if the deque is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the size of the deque
    pub fn size(&self) -> usize {
        self.data.len()
    }
}