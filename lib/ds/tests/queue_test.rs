use ds::queue::Queue;

#[cfg(test)]
mod queue_tests {
    use super::*;

    #[test]
    fn test_queue_enqueue() {
        let mut queue = Queue::new();

        queue.enqueue(123);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_queue_dequeue() {
        let mut queue = Queue::new();

        queue.enqueue(123);
        queue.enqueue(321);

        assert_eq!(queue.is_empty(), false);

        let item = queue.dequeue();

        assert_eq!(queue.len(), 1);
        assert_eq!(item, Some(123));

        let item = queue.dequeue();

        assert_eq!(queue.len(), 0);
        assert_eq!(item, Some(321));

        let item = queue.dequeue();

        assert_eq!(queue.len(), 0);
        assert_eq!(item, None);

        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = Queue::new();

        queue.enqueue(12.32f64);
        queue.enqueue(33.2f64);

        let first = queue.peek().unwrap();

        assert_eq!(*first, 12.32f64);
        assert_eq!(queue.len(), 2);

        let first_mut = queue.peek_mut().unwrap();

        *first_mut = 3.3f64;

        let first = queue.peek().unwrap();
        assert_eq!(*first, 3.3f64);
    }
}
