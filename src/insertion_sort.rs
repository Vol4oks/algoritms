pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort::insertion_sort;

    #[test]
    fn test_empty() {
        let mut v: Vec<i32> = vec![];
        insertion_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut v = vec![5];
        insertion_sort(&mut v);
        assert_eq!(v, vec![5]);
    }

    #[test]
    fn test_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
