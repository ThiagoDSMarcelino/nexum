#[cfg(test)]
mod tests {
    use nexum::LinkedList;

    #[test]
    fn new() {
        let list: LinkedList<u32> = LinkedList::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn from() {
        let arr: [String; 3] = ["a".to_owned(), "b".to_owned(), "c".to_owned()];

        // Test if the from method clone the elements, not just the pointer
        let list = {
            let clone = arr.clone();
            LinkedList::from(clone)
        };

        for (i, elem) in list.iter().enumerate() {
            assert_eq!(&arr[i], elem);
        }

        assert_eq!(Some(&arr[0]), list.front())
    }

    #[test]
    fn default() {
        let list: LinkedList<u32> = LinkedList::default();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
}
