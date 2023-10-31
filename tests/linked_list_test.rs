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
        let arr = [5, 2, 1, 4];
        let list = LinkedList::from(arr);

        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(&arr[i], elem);
        }
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
    fn iter() {
        let arr = [3, 5, 8, 4];
        let list: LinkedList<u32> = LinkedList::from(arr);

        let iter = list.into_iter();

        for (i, elem) in iter.enumerate() {
            assert_eq!(&arr[i], elem);
        }
    }

    #[test]
    fn iter_after_push() {
        let arr = [3, 5, 8, 4];
        let mut vec = arr.to_vec();
        let mut list = LinkedList::from(arr);

        vec.push(16);
        list.push_back(16);

        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(&vec[i], elem);
        }
    }

    #[test]
    fn iter_after_pop() {
        let arr = [3, 5, 8, 4];
        let mut vec = arr.to_vec();
        let mut list = LinkedList::from(arr);

        vec.pop();
        list.pop_back();

        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(&vec[i], elem);
        }
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
    fn insert() {
        let arr = [0, 1, 2, 3, 4, 5];
        let mut vec = arr.to_vec();
        let mut list = LinkedList::from(arr);

        list.insert(2, 9);
        vec.insert(2, 9);

        assert_eq!(list.len(), vec.len());
        for (i, elem) in list.into_iter().enumerate() {
            assert_eq!(elem, &vec[i]);
        }
    }
}
