#[cfg(test)]
mod tests {
    use data_structures::LinkedList;

    #[test]
    fn crete_list() {
        let mut list: LinkedList<u32> = LinkedList::new();

        assert_eq!(list.length, 0);
        assert_eq!(list.get(0), None);
        assert_eq!(list.get(1), None);
    }

    // #[test]
    // fn push_front() {
    //     let mut list: LinkedList<u32> = LinkedList::new();

    //     list.push_front(5);
    //     list.push_front(7);

    //     assert_eq!(list.get(0), Some(&7));
    //     assert_eq!(list.get(1), Some(&5));
    //     assert_eq!(list.get(2), None);
    //     assert_eq!(list.length, 2);
    // }
}
