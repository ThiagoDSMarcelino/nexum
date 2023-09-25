use std::ptr::NonNull;

use super::Node;

#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    pub(super) head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
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
        self.length == 0
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

    pub fn push_front(&mut self, element: T) {
        let new_node = Box::new(Node::new(element));
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

    pub fn push_back(&mut self, element: T) {
        let new_node = Box::new(Node::new(element));
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

            self.length -= 1;
            node.into_element()
        })
    }

    pub fn insert(&mut self, index: usize, element: T) {
        if index > self.length {
            panic!("Invalid index");
        }

        if index == 0 {
            self.push_front(element);
            return;
        }

        if index == self.length {
            self.push_back(element);
            return;
        }

        match self.length / 2 > index {
            true => self.inset_by_head(index, element),
            false => self.inset_by_tail(index, element),
        }
    }

    fn inset_by_head(&mut self, index: usize, element: T) {
        let mut crr = self.head;
        let mut count: usize = 0;

        unsafe {
            loop {
                if index != count {
                    crr = (*crr.unwrap().as_ptr()).next;
                    count += 1;
                    continue;
                }

                self.insert_node(crr, element);
                return;
            }
        }
    }

    fn inset_by_tail(&mut self, index: usize, element: T) {
        let mut crr = self.tail;
        let mut count: usize = self.length;

        unsafe {
            loop {
                if index != count {
                    crr = (*crr.unwrap().as_ptr()).prev;
                    count -= 1;
                    continue;
                }

                self.insert_node(crr, element);
                return;
            }
        }
    }

    unsafe fn insert_node(&mut self, node: Option<NonNull<Node<T>>>, element: T) {
        if let Some(node) = node {
            let mut new_node = Box::new(Node::new(element));

            new_node.prev = (*node.as_ptr()).prev;
            new_node.next = Some(node);

            let node_ptr = NonNull::from(Box::leak(new_node));

            (*(*node.as_ptr()).prev.unwrap().as_ptr()).next = Some(node_ptr);
            (*node.as_ptr()).prev = Some(node_ptr);

            self.length += 1;
        }
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(source: [T; N]) -> Self {
        let mut list = LinkedList::new();

        for element in source {
            list.push_back(element);
        }

        list
    }
}
