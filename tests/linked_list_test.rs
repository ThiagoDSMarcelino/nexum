#[cfg(test)]
mod tests {
    use data_structures::linked_list::LinkedList;

    #[test]
    fn test_get() {
        let mut list: LinkedList<i32> = LinkedList::new(&2);
        assert_eq!(list.get(0), &Some(2));
        assert_eq!(list.get(1), &None);
    }

    #[test]
    fn test_push() {
        let mut list: LinkedList<i32> = LinkedList::new(&2);
        list.push(&3);

        assert_eq!(list.get(0), &Some(2));
        assert_eq!(list.get(1), &Some(3));
    }
}
