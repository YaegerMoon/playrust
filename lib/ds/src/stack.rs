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

// 테스트 모듈 시작
#[cfg(test)] // 'cargo test' 명령어를 실행할 때만 컴파일됨
mod tests {
    use super::*; // 부모 모듈(IntStack 등)의 아이템을 가져옴

    #[test]
    fn test_stack_push() {
        let mut stack = IntStack::new();
        stack.push(1);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = IntStack::new();

        let item = stack.pop();
        assert_eq!(item, None);

        stack.push(1);
        stack.push(2);

        let item = stack.pop();
        assert_eq!(item, Some(2));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = IntStack::new();

        let item = stack.peek();
        assert_eq!(item, None);

        stack.push(1);
        stack.push(2);

        let item = stack.peek().unwrap();
        assert_eq!(*item, 2);
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack = IntStack::new();
        stack.push(1);
        stack.push(2);

        let result = stack.is_empty();
        assert_eq!(result, false);

        stack.pop();
        stack.pop();

        let result = stack.is_empty();
        assert_eq!(result, true)
    }
}
