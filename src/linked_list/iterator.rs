use std::{marker::PhantomData, ptr::NonNull};

use super::LinkedList;

use super::Node;

/// Iterator for traversing the elements of a linked list.
pub struct LinkedListIterator<'a, T> {
    node: Option<&'a NonNull<Node<T>>>,
    index: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    fn new(list: &'a LinkedList<T>) -> Self {
        Self {
            node: list.head.as_ref(),
            index: 0,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    /// Advances the iterator and returns the next element.
    ///
    /// Returns `Some(&'a T)` if there is a next element, or `None` if the end of the list is reached.
    fn next(&mut self) -> Option<&'a T> {
        self.node.map(|crr| unsafe {
            self.node = (*crr.as_ptr()).next.as_ref();
            self.index += 1;

            &(*crr.as_ptr()).element
        })
    }
}

pub struct LinkedListIteratorMut<'a, T> {
    node: Option<&'a mut NonNull<Node<T>>>,
    index: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> LinkedListIteratorMut<'a, T> {
    fn new(list: &'a mut LinkedList<T>) -> Self {
        Self {
            node: list.head.as_mut(),
            index: 0,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for LinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;

    /// Advances the iterator and returns the next element.
    ///
    /// Returns `Some(&'a T)` if there is a next element, or `None` if the end of the list is reached.
    fn next(&mut self) -> Option<&'a mut T> {
        self.node.take().map(|crr| unsafe {
            self.node = (*crr.as_ptr()).next.as_mut();
            self.index += 1;

            &mut (*crr.as_ptr()).element
        })
    }
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(self)
    }

    pub fn iter_mut(&mut self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut::new(self)
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIterator<'a, T>;

    /// Converts a reference to a linked list into an iterator.
    fn into_iter(self) -> LinkedListIterator<'a, T> {
        LinkedListIterator::new(self)
    }
}
