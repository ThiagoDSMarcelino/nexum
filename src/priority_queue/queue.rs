pub struct PriorityQueue<TElement, TPriority: PartialOrd> {
    data: Vec<TElement>,
    priorities: Vec<TPriority>,
}

impl<TElement, TPriority> Default for PriorityQueue<TElement, TPriority>
where
    TPriority: PartialOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<TElement, TPriority> PriorityQueue<TElement, TPriority>
where
    TPriority: PartialOrd,
{
    /// Creates a new [`PriorityQueue<TElement, TPriority>`].
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            priorities: Vec::new(),
        }
    }

    /// Creates a new [`PriorityQueue<TElement, TPriority>`] with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            priorities: Vec::with_capacity(capacity),
        }
    }

    /// Removes all items from the [`PriorityQueue<TElement, TPriority>`].
    pub fn clear(&mut self) {
        self.priorities.clear();
        self.data.clear();
    }

    /// Returns the length of this [`PriorityQueue<TElement, TPriority>`].
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the capacity of this [`PriorityQueue<TElement, TPriority>`].
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns if the [`PriorityQueue<TElement, TPriority>`] is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Removes and returns the minimal element from the [`PriorityQueue<TElement, TPriority>`] - that is, the element with the lowest priority value.
    pub fn dequeue(&mut self) -> Option<TElement> {
        self.priorities.pop();
        self.data.pop()
    }

    /// Removes the minimal element and then immediately adds the specified element with associated priority to the [`PriorityQueue<TElement, TPriority>`].
    pub fn dequeue_enqueue(&mut self, element: TElement, priority: TPriority) -> Option<TElement> {
        let removed = self.dequeue();
        self.enqueue(element, priority);

        removed
    }

    /// Adds the specified element with associated priority to the [`PriorityQueue<TElement, TPriority>`].
    pub fn enqueue(&mut self, element: TElement, priority: TPriority) {
        let mut index = 0;

        while let Some(data_priority) = self.priorities.get(index) {
            if &priority > data_priority {
                break;
            }

            index += 1;
        }

        self.data.insert(index, element);
        self.priorities.insert(index, priority);
    }

    /// Adds the specified element with associated priority to the [`PriorityQueue<TElement, TPriority>`], and immediately removes the minimal element, returning the result.
    pub fn enqueue_dequeue(&mut self, element: TElement, priority: TPriority) -> Option<TElement> {
        self.enqueue(element, priority);
        self.dequeue()
    }

    /// Enqueues a sequence of elements pairs to the [`PriorityQueue<TElement, TPriority>`], all associated with the specified priority.
    pub fn enqueue_range(&mut self, elements: Vec<TElement>, priorities: Vec<TPriority>) {
        for (element, priority) in elements.into_iter().zip(priorities.into_iter()) {
            self.enqueue(element, priority);
        }
    }

    /// Ensures that the [`PriorityQueue<TElement, TPriority>`] can hold up to `capacity` items without further expansion of its backing storage.
    pub fn ensure_capacity(&mut self, capacity: usize) {
        self.data.reserve(capacity);
        self.priorities.reserve(capacity);
    }

    /// Returns the minimal element from the [`PriorityQueue<TElement, TPriority>`] without removing it.
    pub fn peek(&self) -> Option<&TElement> {
        self.data.last()
    }

    /// Sets the capacity to the actual number of items in the [`PriorityQueue<TElement, TPriority>`].
    pub fn trim_to_length(&mut self) {
        self.data.shrink_to_fit();
        self.priorities.shrink_to_fit();
    }
}
