pub struct Set<T> {
    array: Vec<T>,
}

impl<T: PartialEq> Set<T> {
    pub fn new() -> Self {
        Set { array: Vec::new() }
    }

    pub fn add(&mut self, value: T) {
        if !self.contains(&value) {
            self.array.push(value);
        }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(pos) = self.array.iter().position(|x| x == value) {
            self.array.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn contains(&self, value: &T) -> bool {
        self.array.contains(value)
    }
}