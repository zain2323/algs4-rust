/*
Sorts the array entries using the Insertion Sort Algorithm.
 */

use crate::sorting::utils::{less, exch};

pub fn sort<T: Ord + Copy>(arr: &mut Vec<T>) {
    let n = arr.len();
    for i in 1..n {
        for j in (1..=i).rev() {
            if less(arr[j], arr[j-1]) {
                exch(arr, j, j - 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::insertion::sort;
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
