#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: Copy> LinkedList<T>
where
    T: PartialEq,
{
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
            Some(link) => link.iterate_get(count + 1, index),
            None => None,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
            return;
        };

        match self.next.as_mut() {
            Some(link) => link.push(value),
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

        if let Some(link) = self.next.as_mut() {
            link.iterate_collect(vec)
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        self.iterate_pop().1
    }

    fn iterate_pop(&mut self) -> (bool, Option<T>) {
        match self.next.as_mut() {
            Some(link) => {
                let result = link.iterate_pop();

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
            Some(link) => link.count(),
            None => 0,
        };

        if self.value.is_some() {
            count += 1;
        };

        count
    }

    pub fn index_of(&mut self, value: T) -> Option<usize> {
        self.iterate_index_of(value, 0)
    }

    pub fn iterate_index_of(&mut self, value: T, index: usize) -> Option<usize> {
        if self.value == Some(value) {
            return Some(index);
        };

        match self.next.as_mut() {
            Some(link) => link.iterate_index_of(value, index + 1),
            None => None,
        }
    }
}
