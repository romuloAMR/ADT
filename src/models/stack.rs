pub struct Stack<T> {
    data: Vec<T>
}

impl<T> Stack<T> {
    // Create a new stack
    pub fn new() -> Self {
        Stack {
            data: Vec::new()
        }
    }

    // Add an element to the stack
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    // Remove an element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    // Peek at the top element of the stack
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the size of the stack
    pub fn size(&self) -> usize {
        self.data.len()
    }
}