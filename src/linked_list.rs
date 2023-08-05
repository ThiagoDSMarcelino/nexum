#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: Copy> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
        LinkedList {
            value: Some(value),
            next: None,
        }
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        self.iterate_get(0, index)
    }

    fn iterate_get(&mut self, count: usize, index: usize) -> Option<T> {
        if count == index {
            return self.value;
        };

        match self.next.as_mut() {
            Some(node) => node.iterate_get(count + 1, index),
            None => None,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        };

        match self.next.as_mut() {
            Some(node) => node.push(value),
            None => self.next = Some(Box::new(LinkedList::new(value))),
        };
    }

    pub fn collect(&mut self) -> Vec<Option<T>> {
        let mut result = Vec::<Option<T>>::new();

        self.iterate_collect(&mut result);

        result
    }

    fn iterate_collect(&mut self, vec: &mut Vec<Option<T>>) {
        if self.value.is_some() {
            vec.push(self.value);
        };

        if let Some(node) = self.next.as_mut() {
            node.iterate_collect(vec)
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        self.iterate_pop().1
    }

    fn iterate_pop(&mut self) -> (bool, Option<T>) {
        match self.next.as_mut() {
            Some(node) => {
                let result = node.iterate_pop();

                if result.0 {
                    self.next = None;
                }

                (false, result.1)
            }
            None => (true, self.value),
        }
    }

    pub fn count(&mut self) -> usize {
        let mut count: usize = match self.next.as_mut() {
            Some(node) => node.count(),
            None => 0,
        };

        if self.value.is_some() {
            count += 1;
        };

        count
    }
}
