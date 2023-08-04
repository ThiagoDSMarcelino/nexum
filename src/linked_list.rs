#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: Copy> LinkedList<T> {
    pub fn new(value: &T) -> LinkedList<T> {
        LinkedList {
            value: Some(*value),
            next: None,
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

    pub fn push(&mut self, value: &T) {
        if self.value.is_none() {
            self.value = Some(*value);
        };

        match self.next.as_mut() {
            Some(node) => node.push(value),
            None => self.next = Some(Box::new(LinkedList::new(value))),
        };
    }

    pub fn collect(&mut self) -> Vec<&Option<T>> {
        let mut result = Vec::<&Option<T>>::new();
        
        self.iterate_collect(&mut result);

        result
    }

    fn iterate_collect<'a>(&'a mut self, vec: &mut Vec<&'a Option<T>>) {
        if self.value.is_some() {
            vec.push(&self.value);
        };

        match self.next.as_mut() {
            Some(node) => node.iterate_collect(vec),
            None => return,
        };
    }

}
