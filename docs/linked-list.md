# Linked List Methods

## Creation Methods

### `new() -> LinkedList<T>`

Creates an empty linked list.

### `default() -> LinkedList<T>`

Creates an empty linked list with default settings.

### `from(source: [T; N]) -> LinkedList<T>`

Creates a linked list with the elements from the provided array.

## Example of How to Create a Linked List

```rust
use nexum::LinkedList;

fn main() {
    let new_list: LinkedList<u32> = LinkedList::new();
    let default_list: LinkedList<u32> = LinkedList::default();
    let from_list: LinkedList<u32> = LinkedList::from([1, 2, 3]);
}
```

## Get data Methods

### `len(&self) -> usize`

Returns the number of elements in the linked list.

### `is_empty(&self) -> bool`

Returns if the linked list is empty.

### `clear(&mut self)`

Removes all elements from the linked list, making it empty.

### Example of how get data from a linked list

```rust
use nexum::LinkedList;

fn main() {
    let new_list: LinkedList<u32> = LinkedList::new();
    let default_list: LinkedList<u32> = LinkedList::default();
    let from_list: LinkedList<u32> = LinkedList::from([1, 2, 3]);
}
```
