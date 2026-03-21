pub struct IntStack {
    storage: Vec<i32>,
}

impl IntStack {
    pub fn new() -> Self {
        Self { storage: vec![] }
    }

    pub fn push(&mut self, item: i32) {
        self.storage.push(item);
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.storage.pop()
    }

    pub fn peek(&self) -> Option<&i32> {
        // 여기에 코드를 작성하세요.
        self.storage.last()
    }

    // 5. 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        // 여기에 코드를 작성하세요.
        self.storage.is_empty()
    }
}

pub struct Stack<T> {
    storage: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { storage: vec![] }
    }

    pub fn push(&mut self, item: T) {
        self.storage.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.storage.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.storage.last()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}
