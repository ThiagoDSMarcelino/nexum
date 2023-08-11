#[cfg(test)]

mod tests {
    use data_structures::Stack;

    #[test]
    fn test_get() {
        let mut stack = Stack::<f32>::default();
        stack[0] = 0.8;

        assert_eq!(stack[0], 0.8);
        assert_eq!(stack[1], 0.0);
    }
}
