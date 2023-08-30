use std::ptr::NonNull;

#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub const fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(new_node));

        unsafe {
            (*node_ptr.as_ptr()).next = self.head;

            let node = Some(node_ptr);

            match self.head {
                Some(head) => (*head.as_ptr()).prev = node,
                None => self.tail = node,
            }

            self.head = node;
        }

        self.length += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let node_ptr = NonNull::from(Box::leak(new_node));

        unsafe {
            (*node_ptr.as_ptr()).prev = self.tail;

            let node = Some(node_ptr);

            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = node,
                None => self.head = node,
            }

            self.tail = node;
        }

        self.length += 1;
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.map(|node| &(*node.as_ptr()).element) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.map(|node| &(*node.as_ptr()).element) }
    }
}
