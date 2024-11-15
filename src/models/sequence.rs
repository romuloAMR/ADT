pub struct Sequence<T> {
    data: Box<[T]>,
    size: usize,
}

impl<T: Copy + Default> Sequence<T> {
    // Init the ADT
    pub fn new() -> Self {
        Sequence {
            data: vec![T::default(); 8].into_boxed_slice(),
            size: 0,
        }
    }

    // Resize ADT in the heap
    fn resize(&mut self) {
        if self.size == self.data.len() {
            let new_size = self.data.len() + 8;
            let mut new_data = vec![T::default(); new_size].into_boxed_slice();
            for i in 0..self.size {
                new_data[i] = self.data[i];
            }
            self.data = new_data;
        } else if self.size != 0 && self.size <= self.data.len() - 8 {
            let new_size = self.data.len() - 8;
            let mut new_data = vec![T::default(); new_size].into_boxed_slice();
            for i in 0..self.size {
                new_data[i] = self.data[i];
            }
            self.data = new_data;
        }
    }

    // Insert one element
    pub fn add(&mut self, value: T) {
        if self.size == self.data.len() {
            self.resize();
        }
        self.data[self.size] = value;
        self.size += 1;
    }

    // Remove one element
    pub fn remove(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.data[self.size])
        } else {
            None
        }
    }

    // Get one element by index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            Some(&self.data[index])
        } else {
            None
        }
    }

    // Get size from ADT
    pub fn size(&self) -> usize {
        self.size
    }
}