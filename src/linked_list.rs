#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>
}

impl<T> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
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