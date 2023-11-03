use crate::LinkedList;

#[derive(Debug)]
pub struct HashTable<T> {
    pub data: Box<[Box<LinkedList<T>>]>,
}

impl<T: std::default::Default> HashTable<T> {
    const INITIAL_CAPACITY: usize = 10;

    pub fn new() -> Self {
        let mut vec = Vec::with_capacity(HashTable::<T>::INITIAL_CAPACITY);

        for _ in 0..HashTable::<T>::INITIAL_CAPACITY {
            vec.push(Box::new(LinkedList::<T>::new()));
        }

        Self {
            data: vec.into_boxed_slice(),
        }
    }

    pub fn with_capacity(&self, capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            vec.push(Box::new(LinkedList::<T>::new()));
        }

        Self {
            data: vec.into_boxed_slice(),
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for HashTable<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
