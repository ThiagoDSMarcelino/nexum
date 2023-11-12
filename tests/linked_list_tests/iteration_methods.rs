#[cfg(test)]
mod tests {
    use nexum::LinkedList;

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
}
