#[cfg(test)]
mod tests {
    use nexum::PriorityQueue;

    #[test]
    fn test_new() {
        let queue: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let queue: PriorityQueue<i32, i32> = PriorityQueue::with_capacity(10);
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), 10);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = PriorityQueue::new();
        queue.enqueue(1, 1);
        assert_eq!(queue.dequeue(), Some(1));
    }

    #[test]
    fn test_enqueue_dequeue_with_priority() {
        let mut queue = PriorityQueue::new();
        queue.enqueue(1, 2);
        queue.enqueue(2, 1);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(1));
    }

    #[test]
    fn test_peek() {
        let mut queue = PriorityQueue::new();
        assert_eq!(queue.peek(), None);
        queue.enqueue(1, 1);
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn test_clear() {
        let mut queue = PriorityQueue::new();
        queue.enqueue(1, 1);
        queue.clear();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue_range() {
        let mut queue = PriorityQueue::new();
        queue.enqueue_range(vec![1, 2, 3], vec![5, 1, 0]);
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(1));
    }

    #[test]
    fn test_ensure_capacity() {
        let mut queue: PriorityQueue<i32, i32> = PriorityQueue::new();
        queue.ensure_capacity(10);
        assert!(queue.capacity() >= 10);
    }

    #[test]
    fn test_trim_to_length() {
        let mut queue = PriorityQueue::with_capacity(10);
        queue.enqueue_range(vec![1, 2, 3], vec![1, 2, 3]);
        queue.trim_to_length();
        assert!(queue.capacity() <= 4);
    }
}
