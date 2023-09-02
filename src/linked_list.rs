use std::{marker::PhantomData, ptr::NonNull};

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
    const fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }

    fn into_element(self) -> T {
        self.element
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

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    pub fn front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &(*node.as_ptr()).element })
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &(*node.as_ptr()).element })
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

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());

            self.head = node.next;

            match self.head {
                Some(head) => (*head.as_ptr()).prev = None,
                None => self.tail = None,
            }

            self.length -= 1;
            node.into_element()
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());

            self.tail = node.prev;

            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = None,
                None => self.head = None,
            }

            self.length -= 1;
            node.into_element()
        })
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index > self.length {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        if index == self.length {
            return self.pop_back();
        }

        match self.length / 2 > index {
            true => self.remove_by_head(index),
            false => self.remove_by_tail(index),
        }
    }

    fn remove_by_head(&mut self, index: usize) -> Option<T> {
        let mut crr = self.head;
        let mut count: usize = 0;

        unsafe {
            loop {
                if index != count {
                    crr = (*crr.unwrap().as_ptr()).next;
                    count += 1;
                    continue;
                }

                return self.remove_node(crr);
            }
        }
    }

    fn remove_by_tail(&mut self, index: usize) -> Option<T> {
        let mut crr = self.tail;
        let mut count: usize = self.length;

        unsafe {
            loop {
                if index != count {
                    crr = (*crr.unwrap().as_ptr()).prev;
                    count -= 1;
                    continue;
                }

                return self.remove_node(crr);
            }
        }
    }

    unsafe fn remove_node(&mut self, node: Option<NonNull<Node<T>>>) -> Option<T> {
        node.map(|node| {
            let node = Box::from_raw(node.as_ptr());

            if let Some(next) = node.next {
                (*next.as_ptr()).prev = node.prev
            }

            if let Some(prev) = node.prev {
                (*prev.as_ptr()).next = node.next
            }

            node.into_element()
        })
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = NodeIterator<'a, T>;

    fn into_iter(self) -> NodeIterator<'a, T> {
        NodeIterator {
            node: self.head,
            index: 0,
            marker: PhantomData,
        }
    }
}

pub struct NodeIterator<'a, T> {
    node: Option<NonNull<Node<T>>>,
    index: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.node.map(|crr| unsafe {
            self.node = (*crr.as_ptr()).next;
            self.index += 1;

            &(*crr.as_ptr()).element
        })
    }
}
