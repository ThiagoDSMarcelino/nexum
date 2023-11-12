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
    fn push_nulls() {
        let mut vec = AutoExpandVector::new();

        vec.push(1);
        vec.insert(5, 2);

        assert_eq!(vec.capacity(), 6);
        assert_eq!(vec.len(), 6);
        assert_eq!(vec.get(0), &Some(1));
        assert_eq!(vec.get(3), &None);
        assert_eq!(vec.get(5), &Some(2));
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