#[cfg(test)]
mod tests {
    use data_structures::LinkedList;

    #[test]
    fn new_list() {
        let list: LinkedList<u32> = LinkedList::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_front() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_front(5);
        list.push_front(7);

        assert_eq!(list.front(), Some(&7));
        assert_eq!(list.back(), Some(&5));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn push_back() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(9);
        list.push_back(1);
        list.push_back(4);

        assert_eq!(list.front(), Some(&9));
        assert_eq!(list.back(), Some(&4));
        assert_eq!(list.len(), 3);
    }
}
