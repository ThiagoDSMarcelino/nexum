use std::ops::{Index, IndexMut};

use crate::HashTable;

use super::hash_algorithm::HashAlgorithm;

impl<'a, T, H: HashAlgorithm> Index<&'a str> for HashTable<T, H> {
    type Output = T;

    fn index(&self, index: &'a str) -> &Self::Output {
        let hash = H::hash(index, self.data.len());

        &self.data[hash]
            .as_ref()
            .unwrap_or_else(|| panic!("Key not found"))
            .value
    }
}

impl<'a, T, H: HashAlgorithm> IndexMut<&'a str> for HashTable<T, H> {
    fn index_mut(&mut self, index: &'a str) -> &mut Self::Output {
        let hash = H::hash(index, self.data.len());

        &mut self.data[hash]
            .as_mut()
            .unwrap_or_else(|| panic!("Key not found"))
            .value
    }
}
