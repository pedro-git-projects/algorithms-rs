pub fn linear_search<T, U>(container: T, value: U) -> Option<usize>
where
    T: IntoIterator,
    T::Item: PartialEq<U>,
{
    let iter = container.into_iter();

    for (idx, el) in iter.enumerate() {
        if el == value {
            return Some(idx);
        }
    }

    None
}

#[cfg(test)]
mod linear_search_tests {
    use crate::search::linear::linear_search;

    #[test]
    fn test_linear_search_found() {
        let vec = vec![0, 1, 2, 3, 4, 5];
        let result = linear_search(vec, 4);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_linear_search_not_found() {
        let vec = vec![0, 1, 2, 3, 4, 5];
        let result = linear_search(vec, -1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_linear_search_wrong_index() {
        let vec = vec![0, 1, 2, 3, 4, 5];
        let result = linear_search(vec, 0);
        assert_ne!(result, Some(1));
    }
}
