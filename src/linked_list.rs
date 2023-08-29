#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    pub length: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&T> {
        if index > self.length {
            return None;
        }

        match index < self.length / 2 {
            true => self.get_head(index),
            false => self.get_tail(index),
        }
    }

    fn get_head(&mut self, index: usize) -> Option<&T> {
        let mut count: usize = 0;

        let mut crr = self.head.as_ref();

        loop {
            match crr {
                Some(node) => {
                    if index == count {
                        return Some(&node.value);
                    }

                    crr = node.next.as_ref();
                }
                None => return None,
            }

            count += 1;
        }
    }

    fn get_tail(&mut self, index: usize) -> Option<&T> {
        let mut count: usize = self.length;

        let mut crr = self.tail.as_ref();

        loop {
            match crr {
                Some(node) => {
                    if index == count {
                        return Some(&node.value);
                    }

                    crr = node.prev.as_ref();
                }
                None => return None,
            }

            count -= 1;
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node {
            value,
            prev: None,
            next: self.head.clone(),
        };

        let reference = Some(Box::new(new_node));

        if let Some(node) = self.head.as_mut() {
            node.prev = reference.clone();
        }

        self.head = reference;

        if self.tail.is_none() {
            self.tail = self.head.clone();
        }

        self.length += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node {
            value,
            prev: self.tail.clone(),
            next: None,
        };

        self.tail = Some(Box::new(new_node));

        if self.head.is_none() {
            self.head = self.tail.clone();
        }

        self.length += 1;
    }
}
