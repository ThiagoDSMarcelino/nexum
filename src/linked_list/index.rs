use std::ops::{Index, IndexMut};

use crate::LinkedList;

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut count = 0;

        let mut iter = self.iter();

        loop {
            let node = iter.next();

            if node.is_none() {
                panic!("Index out of bounds");
            }

            if count == index {
                return node.unwrap();
            }

            count += 1;
        }
    }
}


impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut count = 0;

        let mut iter = self.iter_mut();

        loop {
            let node = iter.next();

            if node.is_none() {
                panic!("Index out of bounds");
            }

            if count == index {
                return node.unwrap();
            }

            count += 1;
        }
    }
}
