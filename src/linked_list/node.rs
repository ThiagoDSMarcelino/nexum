use std::ptr::NonNull;


/// # Node
/// Represents a node in a doubly linked list.
///
/// This `struct` is used to create and manage nodes within a linked list. Each node contains
/// an element of generic type `T`, as well as optional references to the next and previous
/// nodes in the list.
#[derive(Debug, Clone)]
pub(super) struct Node<T> {
    /// The element stored in the node.
    pub(super) element: T,
    /// A reference to the next node in the list, if it exists.
    pub(super) next: Option<NonNull<Node<T>>>,
    /// A reference to the previous node in the list, if it exists.
    pub(super) prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    /// Create a new Node with the specified element and no next or previous references.
    ///
    /// # Arguments
    ///
    /// * `element` - The value to be stored in the node.
    ///
    /// # Returns
    ///
    /// A new Node with the given element and no next or previous references.
    pub(super) const fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }

    /// Consume a node and return its stored element.
    ///
    /// # Returns
    ///
    /// The element stored in the node.
    pub(super) fn into_element(self) -> T {
        self.element
    }
}
