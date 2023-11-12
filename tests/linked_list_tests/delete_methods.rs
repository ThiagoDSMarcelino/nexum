#[cfg(test)]
mod tests {
    use nexum::LinkedList;

    #[test]
    fn pop_front() {
        let mut list: LinkedList<u32> = LinkedList::from([12, 6, 1]);

        let popped = list.pop_front();

        assert_eq!(popped, Some(12));
        assert_eq!(list.front(), Some(&6));
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_front_should_left_one_item() {
        let mut list: LinkedList<u32> = LinkedList::from([12, 6, 1]);

        for _ in 1..list.len() {
            list.pop_front();
        }

        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn pop_front_should_clear_list() {
        let mut list: LinkedList<u32> = LinkedList::from([12, 6, 1]);

        for _ in 0..list.len() {
            list.pop_front();
        }

        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn pop_back() {
        let mut list: LinkedList<u32> = LinkedList::from([12, 6, 1]);

        let popped = list.pop_back();

        assert_eq!(popped, Some(1));
        assert_eq!(list.front(), Some(&12));
        assert_eq!(list.back(), Some(&6));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_back_should_left_one_item() {
        let mut list: LinkedList<u32> = LinkedList::from([12, 6, 1]);

        for _ in 1..list.len() {
            list.pop_back();
        }

        assert_eq!(list.front(), Some(&12));
        assert_eq!(list.back(), Some(&12));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn pop_back_should_clear_list() {
        let mut list: LinkedList<u32> = LinkedList::from([12, 6, 1]);

        for _ in 0..list.len() {
            list.pop_back();
        }

        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn remove() {
        let arr = [0, 1, 2, 3, 4, 5];
        let mut vec = arr.to_vec();
        let mut list = LinkedList::from(arr);

        let removed_list = list.remove(2);
        let removed_vec = vec.remove(2);

        assert_eq!(list.len(), vec.len());
        assert_eq!(removed_list, Some(removed_vec));
        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(elem, &vec[i]);
        }
    }

    #[test]
    fn clear() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.clear();

        assert!(list.is_empty());
    }
}
