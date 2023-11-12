#[cfg(test)]
mod tests {
    use nexum::HashTable;

    #[test]
    fn new() {
        let hash = HashTable::<i32>::new();

        assert!(hash.is_empty());
        assert_eq!(hash.get_hash_algorithm(), "Djb2");
        assert_eq!(hash.capacity(), 10);
    }

    #[test]
    fn default() {
        let hash = HashTable::<i32>::default();

        assert!(hash.is_empty());
        assert_eq!(hash.get_hash_algorithm(), "Djb2");
        assert_eq!(hash.capacity(), 10);
    }

    #[test]
    fn with_capacity() {
        let hash = HashTable::<i32>::with_capacity(100);

        assert!(hash.is_empty());
        assert_eq!(hash.get_hash_algorithm(), "Djb2");
        assert_eq!(hash.capacity(), 100);
    }

    #[test]
    fn insert() {
        let mut hash = HashTable::<i32>::new();

        hash.insert("hello".to_string(), 1);
        hash.insert("world".to_string(), 2);

        assert!(!hash.is_empty());
        assert_eq!(hash["hello"], 1);
        assert_eq!(hash["world"], 2);
    }

    #[test]
    fn set() {
        let mut hash = HashTable::<i32>::new();

        hash.set("hello".to_string(), 1);
        hash.set("world".to_string(), 2);
        hash.set("hello".to_string(), 3);

        assert!(!hash.is_empty());
        assert_eq!(hash["hello"], 3);
        assert_eq!(hash["world"], 2);
    }

    #[test]
    #[should_panic]
    fn index_panic() {
        let hash = HashTable::<i32>::new();
        let _ = hash["panic"];
    }

    #[test]
    #[should_panic]
    fn index_mut_panic() {
        let mut hash = HashTable::<i32>::new();
        hash["panic"] = 1;
    }
}
