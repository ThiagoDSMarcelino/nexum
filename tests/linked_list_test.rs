#[cfg(test)]
mod tests {
    use data_structures::linked_list::LinkedList;

    #[test]
    fn test_get() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new(2);

        assert_eq!(list.get(0), &Some(2));
        assert_eq!(list.get(1), &None);
    }
}