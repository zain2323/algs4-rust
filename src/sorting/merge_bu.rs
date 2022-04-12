/*
Sorts the array entries using the Merge Sort Algorithm.
Uses the non-recursive bottom up approach
 */

use std::cmp::min;
use std::fmt::Debug;
use crate::sorting::utils::{less, exch};

// Merges the two sorted arrays
fn merge<T: PartialOrd + Clone>(arr: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    //     Copy a[lo..hi] to aux[lo..hi]
    for k in lo..=hi {
        aux[k] =  arr[k].clone();
    }

    let mut i = lo;
    let mut j = mid+1;
    for k in lo..=hi {
        if i > mid {
            arr[k] =  aux[j].clone();
            j += 1;
        }
        else if j > hi {
            arr[k] =  aux[i].clone();
            i += 1;
        }
        else if less(&aux[j], &aux[i]) {
            arr[k] =  aux[j].clone();
            j += 1;
        }
        else {
            arr[k] =  aux[i].clone();
            i += 1;
        }
    }
}

pub fn sort<T: PartialOrd + Clone + Copy + Debug>(arr: &mut Vec<T>) {
    let n = arr.len();
    let mut aux: Vec<T> = vec![arr[0]; n];
    let mut len = 1;
    while len < n {
        let mut lo = 0;
        while lo < n - len {
            let mid = lo + len - 1;
            let hi = min(lo + len + len - 1, n - 1);
            merge(arr, &mut aux, lo, mid, hi);
            lo += len + len;
        }
        len *= 2;
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::merge_bu::sort;
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
