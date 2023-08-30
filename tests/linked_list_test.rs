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

        list.pop_front();

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

        list.pop_back();

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

        let mut i: usize = 0;

        let iter = list.into_iter();

        for elem in iter {
            assert_eq!(&vec[i], elem);
            i += 1;
        }
    }
}
