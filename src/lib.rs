mod linked_list;
#[derive(Debug, Clone, Default)]
pub struct LinkedList<T> {
    value: Option<T>,
    next: Option<Box<LinkedList<T>>>,
}

mod stack;
#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    values: [T; 8],
}