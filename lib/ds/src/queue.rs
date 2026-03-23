use std::collections::VecDeque;

pub struct Queue<T> {
    storage: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            storage: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: VecDeque::with_capacity(capacity),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.storage.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.storage.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.storage.front()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.storage.front_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}
