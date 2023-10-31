#[derive(Debug)]
pub struct HashTable<T> {
    data: Box<[T]>,
    capacity: usize,
}

impl<T> HashTable<T>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            data: Box::new([Default::default(); 10]),
            capacity: 10,
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
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
