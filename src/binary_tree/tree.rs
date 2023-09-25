use std::ptr::NonNull;

use super::Node;

#[derive(Debug, Clone)]
pub struct BinaryTree<T: PartialOrd> {
    root: Option<NonNull<Node<T>>>,
    length: usize,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub const fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.length = 0;
    }

    pub fn insert(&mut self, element: T) {
        match self.root {
            Some(node) => unsafe { self.choose(element, node) },
            None => self.root = Some(self.create_node(element)),
        };

        self.length += 1;
    }

    unsafe fn insert_left(&mut self, element: T, node: *mut Node<T>) {
        match (*node).left {
            Some(node) => self.choose(element, node),
            None => (*node).left = Some(self.create_node(element)),
        }
    }

    unsafe fn insert_right(&mut self, element: T, node: *mut Node<T>) {
        match (*node).right {
            Some(node) => self.choose(element, node),
            None => (*node).right = Some(self.create_node(element)),
        }
    }

    unsafe fn choose(&mut self, element: T, node: NonNull<Node<T>>) {
        let node = node.as_ptr();

        if (*node).element <= element {
            self.insert_left(element, node);
            return;
        }

        self.insert_right(element, node)
    }

    fn create_node(&self, element: T) -> NonNull<Node<T>> {
        let new_node = Box::new(Node::new(element));
        let node_ptr: NonNull<Node<T>> = NonNull::from(Box::leak(new_node));

        node_ptr
    }
}
