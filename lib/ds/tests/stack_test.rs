use ds::stack::IntStack;
use ds::stack::Stack;

// 테스트 모듈 시작
#[cfg(test)] // 'cargo test' 명령어를 실행할 때만 컴파일됨
mod int_stack_tests {
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

#[cfg(test)] // 'cargo test' 명령어를 실행할 때만 컴파일됨
mod generic_stack_tests {
    use super::*;

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::new();

        stack.push(3.3);
        stack.push(4.4);
        stack.push(5.5);

        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack::new();

        let item = stack.pop();
        assert_eq!(item, None);

        stack.push(true);
        stack.push(true);

        let item = stack.pop();
        assert_eq!(item, Some(true));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();

        let item = stack.peek();
        assert_eq!(item, None);

        stack.push("a");
        stack.push("B");

        let item = stack.peek().unwrap();
        assert_eq!(*item, "B");
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack = Stack::new();
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
