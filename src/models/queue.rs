pub struct Queue<T> {
    data: Vec<T>
}

impl<T> Queue<T> {
    // Create a new queue
    pub fn new() -> Self {
        Queue {
            data: Vec::new()
        }
    }

    // Add an element to the queue
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    // Remove an element from the queue
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    // Peek at the top element of the queue
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    // Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the size of the queue
    pub fn size(&self) -> usize {
        self.data.len()
    }
}