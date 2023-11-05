#[cfg(test)]
mod tests {
    use nexum::AutoExpandVector;

    #[test]
    fn new() {
        let hash_table: AutoExpandVector<i32> = AutoExpandVector::new();
        assert_eq!(hash_table.capacity(), 0);
    }
}