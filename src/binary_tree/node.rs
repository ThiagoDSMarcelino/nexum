use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub(super) struct Node<T: PartialOrd> {
    pub(super) element: T,
    pub(super) left: Option<NonNull<Node<T>>>,
    pub(super) right: Option<NonNull<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    pub(super) const fn new(element: T) -> Self {
        Self {
            element,
            left: None,
            right: None,
        }
    }
}
