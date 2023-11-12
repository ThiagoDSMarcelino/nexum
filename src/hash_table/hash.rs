use std::{any::type_name, marker::PhantomData};

use super::{
    hash_algorithm::{Djb2, HashAlgorithm},
    node::Node,
};

#[derive(Debug, Clone)]
/// A hash table data structure
pub struct HashTable<T, H: HashAlgorithm = Djb2> {
    pub(super) data: Box<[Option<Node<T>>]>,
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

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|node| node.is_none())
    }

    pub fn get_hash_algorithm(&self) -> &str {
        type_name::<H>().split("::").last().unwrap()
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

    fn gen_node(&mut self, index: usize, key: String, value: T) {
        let new_node = Node::new(key, value);
        self.data[index] = Some(new_node);
    }
}

/// Implementing the Default trait for the HashTable struct
impl<T, H: HashAlgorithm> Default for HashTable<T, H> {
    fn default() -> Self {
        Self::new()
    }
}
