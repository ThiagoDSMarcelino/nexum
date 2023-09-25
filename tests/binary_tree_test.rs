#[cfg(test)]
mod tests {
    use data_structures::BinaryTree;

    #[test]
    fn new() {
        let tree: BinaryTree<u32> = BinaryTree::new();

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn insert() {
        let mut tree: BinaryTree<u32> = BinaryTree::new();

        tree.insert(5);

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
    }
}
