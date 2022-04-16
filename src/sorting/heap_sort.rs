use crate::fundamentals::heap::MinHeap;

fn heap_construction<T: Ord + Copy>(arr: &mut Vec<T>) -> MinHeap<T> {
    let mut heap = MinHeap::<T>::new();
    let n = arr.len();
    for i in 0..n {
        heap.insert(arr[i])
    }
    heap
}
pub fn sort<T: Ord + Copy>(arr: &mut Vec<T>) {
    let mut heap = heap_construction(arr);
    for i in 0..arr.len() {
        arr[i] = heap.del_min();
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::heap_sort::sort;
    use crate::sorting::utils::is_sorted;

    #[test]
    fn test1_sort_random() {
        let mut list = vec![100, 21, 24, 199, 221, 1021];
        sort(&mut list);
        assert_eq!(is_sorted(&list), true);
    }

    #[test]
    fn test2_sort_descending() {
        let mut list = vec![1021, 221, 199, 100, 24, 21];
        sort(&mut list);
        assert_eq!(is_sorted(&list), true);
    }

    #[test]
    fn test3_sort_already() {
        let mut list = vec![21, 24, 100, 199, 221, 1021];
        sort(&mut list);
        assert_eq!(is_sorted(&list), true);
    }
}
