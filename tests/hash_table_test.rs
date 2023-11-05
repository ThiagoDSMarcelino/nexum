#[cfg(test)]
mod tests {
    use nexum::HashTable;

    #[test]
    fn new() {
        let hash_table: HashTable<i32> = HashTable::new();
        assert_eq!(hash_table.capacity(), 10);
    }
}
