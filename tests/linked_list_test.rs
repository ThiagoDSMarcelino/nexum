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

        let sla = list.collect();
        assert_eq!(sla, [&Some(2), &Some(3)]);
    }

    #[test]
    fn test_collect() {
        let mut list: LinkedList<i32> = LinkedList::new(&2);
        list.push(&3);
        list.push(&5);
        list.push(&1);


        let vec = list.collect();
        assert_eq!(vec, [&Some(2), &Some(3), &Some(5), &Some(1)]);
    }
}
