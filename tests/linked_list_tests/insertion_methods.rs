#[cfg(test)]
mod tests {
    use nexum::LinkedList;

    #[test]
    fn push_front() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_front(5);
        list.push_front(7);
        list.push_front(35);

        assert_eq!(list.front(), Some(&35));
        assert_eq!(list[1], 7);
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
        assert_eq!(list[1], 1);
        assert_eq!(list.back(), Some(&4));
        assert_eq!(list.len(), 3);
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
