#[cfg(test)]
mod tests {
    use data_structures::LinkedList;

    #[test]
    fn new() {
        let list: LinkedList<u32> = LinkedList::new();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn default() {
        let list: LinkedList<u32> = LinkedList::default();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
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

    #[test]
    fn push_front() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_front(5);
        list.push_front(7);
        list.push_front(35);

        assert_eq!(list.front(), Some(&35));
        assert_eq!(list.back(), Some(&5));
        assert_eq!(list.len(), 3);
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

    #[test]
    fn pop_front() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_front(1);
        list.push_front(6);
        list.push_front(12);

        let popped = list.pop_front();

        assert_eq!(popped, Some(12));
        assert_eq!(list.front(), Some(&6));
        assert_eq!(list.back(), Some(&1));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_back() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(18);
        list.push_back(56);
        list.push_back(33);

        let popped = list.pop_back();

        assert_eq!(popped, Some(33));
        assert_eq!(list.front(), Some(&18));
        assert_eq!(list.back(), Some(&56));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn iter() {
        let vec = [3, 5, 8, 4];
        let mut list: LinkedList<u32> = LinkedList::new();

        for i in vec {
            list.push_back(i);
        }

        let iter = list.into_iter();

        for (i, elem) in iter.enumerate() {
            assert_eq!(&vec[i], elem);
        }
    }

    #[test]
    fn remove() {
        let mut list = LinkedList::new();
        let mut vec = vec![0, 1, 2, 3, 4, 5];

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        let removed_list = list.remove(2);
        let removed_vec = vec.remove(2);

        assert_eq!(removed_list, Some(removed_vec));
        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(elem, &vec[i]);
        }
    }

    #[test]
    fn from() {
        let arr = [5, 2, 1, 4];
        let list = LinkedList::from(arr);

        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(&arr[i], elem);
        }
    }
}
