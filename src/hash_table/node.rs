use std::ptr::NonNull;

#[derive(Clone, Debug)]
pub(super) struct Node<T> {
    key: String,
    value: T,
    pub(super) next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(key: String, value: T) -> Self {
        Self {
            key,
            value,
            next: None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        if self.key == key {
            return Some(&self.value);
        }

        match &self.next {
            Some(node) => unsafe { node.as_ref().get(key) },
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        if self.key == key {
            return Some(&mut self.value);
        }

        match &mut self.next {
            Some(node) => unsafe { node.as_mut().get_mut(key) },
            None => None,
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        match &mut self.next {
            Some(node) => unsafe {
                node.as_mut().insert(key, value);
            },
            None => self.gen_next_node(key, value),
        }
    }

    pub fn insert_or_update(&mut self, key: &str, value: T) {
        if self.key == key {
            self.value = value;
            return;
        }

        match &mut self.next {
            Some(node) => unsafe {
                node.as_mut().insert_or_update(key, value);
            },
            None => self.gen_next_node(key.to_string(), value),
        }
    }

    fn gen_next_node(&mut self, key: String, value: T) {
        let new_node = Box::new(Node::new(key, value));
        let node_ptr = NonNull::from(Box::leak(new_node));

        self.next = Some(node_ptr);
    }
}
