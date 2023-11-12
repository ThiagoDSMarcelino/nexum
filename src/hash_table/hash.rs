use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use super::{
    hash_algorithm::{Djb2, HashAlgorithm},
    node::Node,
};

#[derive(Debug, Clone)]
/// A hash table data structure
pub struct HashTable<T, H: HashAlgorithm = Djb2> {
    data: Box<[Option<Node<T>>]>,
    marker: PhantomData<H>,
}

impl<T, H: HashAlgorithm> HashTable<T, H> {
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 10;

        let mut vec: Vec<Option<Node<T>>> = Vec::with_capacity(INITIAL_CAPACITY);

        for _ in 0..vec.capacity() {
            vec.push(None);
        }

        Self {
            data: vec.into_boxed_slice(),
            marker: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec: Vec<Option<Node<T>>> = Vec::with_capacity(capacity);

        for _ in 0..vec.capacity() {
            vec.push(None);
        }

        Self {
            data: vec.into_boxed_slice(),
            marker: PhantomData,
        }
    }
}

impl<T, H: HashAlgorithm> HashTable<T, H> {
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|node| node.is_none())
    }
}

impl<T, H: HashAlgorithm> HashTable<T, H> {
    pub fn insert(&mut self, key: String, value: T) {
        let index = H::hash(&key, self.data.len());

        match &mut self.data[index] {
            Some(node) => node.insert(key, value),
            None => self.gen_node(index, key, value),
        }
    }

    pub fn set(&mut self, key: String, value: T) {
        let index = H::hash(&key, self.data.len());

        match &mut self.data[index] {
            Some(node) => node.insert_or_update(&key, value),
            None => self.gen_node(index, key, value),
        }
    }
}

impl<T, H: HashAlgorithm> HashTable<T, H> {
    fn gen_node(&mut self, index: usize, key: String, value: T) {
        let new_node = Node::new(key, value);
        self.data[index] = Some(new_node);
    }
}

impl<'a, T, H: HashAlgorithm> Index<&'a str> for HashTable<T, H> {
    type Output = T;

    fn index(&self, index: &'a str) -> &Self::Output {
        let hash = H::hash(index, self.data.len());

        self.data[hash]
            .as_ref()
            .and_then(|node| node.get(index))
            .unwrap_or_else(|| panic!("Key not found"))
    }
}

impl<'a, T, H: HashAlgorithm> IndexMut<&'a str> for HashTable<T, H> {
    fn index_mut(&mut self, index: &'a str) -> &mut Self::Output {
        let hash = H::hash(index, self.data.len());

        self.data[hash]
            .as_mut()
            .and_then(|node| node.get_mut(index))
            .unwrap_or_else(|| panic!("Key not found"))
    }
}

/// Implementing the Default trait for the HashTable struct
impl<T, H: HashAlgorithm> Default for HashTable<T, H> {
    fn default() -> Self {
        Self::new()
    }
}
