#[cfg(test)]
mod tests {
    use nexum::AutoExpandVector;

    #[test]
    fn new() {
        let vec: AutoExpandVector<i32> = AutoExpandVector::new();
        assert_eq!(vec.capacity(), 0);
    }

    #[test]
    fn push() {
        let mut vec = AutoExpandVector::new();
        vec.push(1);
        assert_eq!(vec.capacity(), 1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.get(0), &Some(1));
    }

    #[test]
    fn clear() {
        let mut vec = AutoExpandVector::new();
        vec.push(1);
        vec.clear();
        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.get(0), &None);
    }
}