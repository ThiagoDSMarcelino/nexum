use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    values: [T; 8],
}

impl<T> Index<usize> for Stack<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.values[i]
    }
}

impl<T> IndexMut<usize> for Stack<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.values[i]
    }
}
