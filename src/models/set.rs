pub struct Set<T> {
    array: Vec<T>,
}

impl<T: PartialEq> Set<T> {
    // Create Set
    pub fn new() -> Self {
        Set { array: Vec::new() }
    }

    // Add element
    pub fn add(&mut self, value: T) {
        if !self.contains(&value) {
            self.array.push(value);
        }
    }

    // Remove element
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(pos) = self.array.iter().position(|x| x == value) {
            self.array.remove(pos);
            true
        } else {
            false
        }
    }

    // Verifies that the element is contained
    pub fn contains(&self, value: &T) -> bool {
        self.array.contains(value)
    }

    // Get size
    pub fn size(&self) -> usize {
        self.array.len()
    }
}
