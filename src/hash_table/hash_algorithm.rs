pub trait HashAlgorithm {
    fn hash(key: &str, size: usize) -> usize;
}

pub struct Djb2;
impl HashAlgorithm for Djb2 {
    fn hash(key: &str, size: usize) -> usize {
        let mut hash: usize = 5381;

        for c in key.chars() {
            let c = c as usize;
            hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(c);
        }

        hash % size
    }
}
