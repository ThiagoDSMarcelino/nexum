#[cfg(test)]
mod tests {
    use nexum::HashTable;

    #[test]
    fn new() {
        let hash = HashTable::<i32>::new();

        assert!(hash.is_empty());
    }

    #[test]
    fn with_capacity() {
        let hash = HashTable::<i32>::with_capacity(10);

        assert!(hash.is_empty());
    }

    #[test]
    fn insert() {
        let mut hash = HashTable::<i32>::new();

        hash.insert("hello".to_string(), 1);
        hash.insert("world".to_string(), 2);

        assert!(!hash.is_empty());
        assert!(hash["hello"] == 1);
        assert!(hash["world"] == 2);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let hash = HashTable::<i32>::new();
        let _ = hash["panic"];
    }

    #[test]
    fn set() {
        let mut hash = HashTable::<i32>::new();

        hash.set("hello".to_string(), 1);
        hash.set("world".to_string(), 2);
        hash.set("hello".to_string(), 3);

        assert!(!hash.is_empty());
        assert!(hash["hello"] == 3);
        assert!(hash["world"] == 2);
    }
}
