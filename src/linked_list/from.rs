use crate::LinkedList;

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    /// Creates a linked list from an array.
    fn from(source: [T; N]) -> Self {
        let mut list = LinkedList::new();

        for element in source {
            list.push_back(element);
        }

        list
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    /// Creates a linked list from a vector.
    fn from(source: Vec<T>) -> Self {
        let mut list = LinkedList::new();

        for element in source {
            list.push_back(element);
        }

        list
    }
}

impl<T> From<&[T]> for LinkedList<T>
where
    T: Copy,
{
    /// Creates a linked list from a slice.
    fn from(source: &[T]) -> Self {
        let mut list = LinkedList::new();

        for element in source {
            list.push_back(*element);
        }

        list
    }
}

impl<T> From<&Vec<T>> for LinkedList<T>
where
    T: Copy,
{
    /// Creates a linked list from a reference to a vector.
    fn from(source: &Vec<T>) -> Self {
        let mut list = LinkedList::new();

        for element in source {
            list.push_back(*element);
        }

        list
    }
}

impl<T> From<&LinkedList<T>> for LinkedList<T>
where
    T: Copy,
{
    /// Creates a linked list from a reference to another linked list.
    fn from(source: &LinkedList<T>) -> Self {
        let mut list = LinkedList::new();

        for element in source {
            list.push_back(*element);
        }

        list
    }
}
