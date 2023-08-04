pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            value: None,
            next: None
        }
    }

    pub fn new_with(value: T) -> LinkedList<T> {
        LinkedList {
            value: Some(value),
            next: None
        }
    }

    pub fn get(&mut self, index: usize) -> &Option<T> {
        self.iterate_get(0, index)
    }

    fn iterate_get(&mut self, count: usize, index: usize) -> &Option<T> {
        if count == index {
            return &self.value;
        };

        match self.next.as_mut() {
            Some(node) => node.iterate_get(count + 1, index),
            None => &None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new_with(2);

        assert_eq!(list.get(0), &Some(2));
        assert_eq!(list.get(1), &None);
    }
}