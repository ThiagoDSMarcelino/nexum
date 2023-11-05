use crate::AutoExpandVector;

impl<T, const N: usize> From<[T; N]> for AutoExpandVector<T> {
    fn from(source: [T; N]) -> Self {
        let mut data = Vec::new();

        for element in source {
            data.push(Some(element));
        }

        Self { data }
    }
}

impl<T> From<Vec<T>> for AutoExpandVector<T> {
    fn from(source: Vec<T>) -> Self {
        let mut data = Vec::new();

        for element in source {
            data.push(Some(element));
        }

        Self { data }
    }
}

impl<T> FromIterator<T> for AutoExpandVector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut data = Vec::new();

        for element in iter {
            data.push(Some(element));
        }

        Self { data }
    }
}
