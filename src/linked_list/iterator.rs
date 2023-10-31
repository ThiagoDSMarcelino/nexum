use std::{marker::PhantomData, ptr::NonNull};

use super::LinkedList;

use super::Node;

/// Iterator for traversing the elements of a linked list.
pub struct NodeIterator<'a, T> {
    node: Option<NonNull<Node<T>>>,
    index: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = NodeIterator<'a, T>;

    /// Converts a reference to a linked list into an iterator.
    fn into_iter(self) -> NodeIterator<'a, T> {
        NodeIterator {
            node: self.head,
            index: 0,
            marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    /// Advances the iterator and returns the next element.
    ///
    /// Returns `Some(&'a T)` if there is a next element, or `None` if the end of the list is reached.
    fn next(&mut self) -> Option<&'a T> {
        self.node.map(|crr| unsafe {
            self.node = (*crr.as_ptr()).next;
            self.index += 1;

            &(*crr.as_ptr()).element
        })
    }
}
