use std::ops::Range;

/// A vector that automatically expands to fit new elements.
pub struct AutoExpandVector<T> {
    pub(super) data: Vec<Option<T>>,
}

// Creation methods
impl<T> AutoExpandVector<T> {
    /// Creates a new [`AutoExpandVector<T>`].
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(0),
        }
    }

    /// Creates a new [`AutoExpandVector<T>`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

// Query methods
impl<T> AutoExpandVector<T> {
    /// Returns the capacity of this [`AutoExpandVector<T>`].
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the length of this [`AutoExpandVector<T>`].
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if this [`AutoExpandVector<T>`] is empty.
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Clears this [`AutoExpandVector<T>`].
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.shrink_to_fit();
    }

    /// Returns a reference to the element at the given index.
    pub fn get(&self, index: usize) -> &Option<T> {
        if index >= self.len() {
            return &None;
        }

        &self.data[index]
    }
}

// Mutation methods
impl<T> AutoExpandVector<T> {
    /// Pushes an element into this [`AutoExpandVector<T>`].
    pub fn push(&mut self, element: T) {
        self.data.push(Some(element));
        self.trim_to_length();
    }

    /// Inserts an element into this [`AutoExpandVector<T>`] at the given index.
    pub fn insert(&mut self, index: usize, element: T) {
        self.fill_with_none(index);
        self.data.insert(index, Some(element));
        self.trim_to_length();
    }

    /// Set the element at the given index to the given element.
    pub fn set(&mut self, index: usize, element: T) -> Option<T> {
        if index >= self.len() {
            self.insert(index, element);
            return None;
        }

        let temp = self.data[index].take();
        self.data[index] = Some(element);
        self.trim_to_length();
        temp
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let temp = self.data.remove(index);
        self.trim_to_length();
        temp
    }

    pub fn remove_range(&mut self, range: Range<usize>) {
        self.data.drain(range);
        self.trim_to_length();
    }
}

// Utility methods
impl<T> AutoExpandVector<T> {
    /// Fills this [`AutoExpandVector<T>`] with `None` elements until the given index is reached.
    fn fill_with_none(&mut self, index: usize) {
        while index >= self.len() {
            self.data.push(None);
        }

        self.trim_to_length();
    }

    // TODO: Improve performance
    /// Removes all `None` elements from the start and end of this [`AutoExpandVector<T>`].
    fn trim_to_length(&mut self) {
        while let Some(None) = self.data.first() {
            self.data.remove(0);
        }

        while let Some(None) = self.data.last() {
            self.data.pop();
        }

        self.data.shrink_to_fit();
    }
}

impl<T: PartialEq> AutoExpandVector<T> {
    /// Removes the first element that is equal to the given element.
    pub fn remove_element(&mut self, element: T) -> bool {
        let element = &Some(element);
        let index = self.data.iter().position(|e| e == element);

        if index.is_none() {
            return false;
        }

        self.remove(index.unwrap());
        self.trim_to_length();
        true
    }
}

impl<T: Copy> AutoExpandVector<T> {
    /// Pushes all elements from a slice into this [`AutoExpandVector<T>`].
    pub fn push_all(&mut self, elements: &[T]) {
        for element in elements {
            self.push(*element);
        }

        self.trim_to_length();
    }

    /// Inserts all elements from a slice into this [`AutoExpandVector<T>`] at the given index.
    pub fn insert_all(&mut self, index: usize, elements: &[T]) {
        self.fill_with_none(index);

        for element in elements {
            self.data.insert(index, Some(*element));
        }

        self.trim_to_length();
    }
}

impl<T> Default for AutoExpandVector<T> {
    /// Creates a new [`AutoExpandVector<T>`] with a capacity of 0.
    fn default() -> Self {
        Self::new()
    }
}
