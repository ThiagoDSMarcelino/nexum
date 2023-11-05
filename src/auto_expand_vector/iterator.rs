use std::vec::IntoIter;

use crate::AutoExpandVector;

impl<T> IntoIterator for AutoExpandVector<T> {
    type Item = Option<T>;
    type IntoIter = IntoIter<Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
