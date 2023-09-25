use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub(super) struct Node<T> {
    pub(super) element: T,
    pub(super) next: Option<NonNull<Node<T>>>,
    pub(super) prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub(super) const fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }

    pub(super) fn into_element(self) -> T {
        self.element
    }
}
