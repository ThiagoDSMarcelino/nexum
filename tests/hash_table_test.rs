#[cfg(test)]
mod tests {
    use nexum::HashTable;

    #[test]
    fn test_hash_table_new() {
        let hash_table: HashTable<i32> = HashTable::new();
        assert_eq!(hash_table.capacity(), 10);
    }
}
