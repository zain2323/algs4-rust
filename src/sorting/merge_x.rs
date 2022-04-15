/*
This is the optimized version of Merge Sort algorithm.
 */

use crate::sorting::utils::{less, exch};

const CUTT_OFF: usize = 7;

// Merges the two sorted arrays
fn merge<T: Ord + Copy>(arr: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    //     Copy a[lo..hi] to aux[lo..hi]
    for k in lo..=hi {
        aux[k] =  arr[k];
    }

    let mut i = lo;
    let mut j = mid+1;
    for k in lo..=hi {
        if i > mid {
            arr[k] =  aux[j];
            j += 1;
        }
        else if j > hi {
            arr[k] =  aux[i];
            i += 1;
        }
        else if less(aux[j], aux[i]) {
            arr[k] =  aux[j];
            j += 1;
        }
        else {
            arr[k] =  aux[i];
            i += 1;
        }
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut Vec<T>) {
    let mut aux: Vec<T> = vec![arr[0]; arr.len()];
    merge_sort(&mut aux, arr, 0, arr.len()-1);
}

fn merge_sort<T: Ord + Copy>(arr: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, hi: usize) {
    if hi <= lo + CUTT_OFF {
        insertion_sort(aux, lo, hi);
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort(aux, arr, lo, mid);
    merge_sort(aux, arr, mid+1, hi);

    if !less(arr[mid+1], arr[mid]) {
        for i in lo..=hi {
            aux[i] = arr[i];
        }
        return;
    }
    merge(arr, aux, lo, mid, hi);
}

// sort from a[lo] to a[hi] using insertion sort
fn insertion_sort<T: Ord + Copy>(arr: &mut Vec<T>, lo: usize, hi: usize) {
    for i in lo+1..=hi {
        for j in (lo+1..=i).rev() {
            if less(arr[j], arr[j-1]) {
                exch(arr, j, j - 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::merge_x::{insertion_sort, sort};
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

    #[test]
    fn test4_insertion_sort() {
        let mut list = vec![21, 24, 100, 199, 221, 1021];
        insertion_sort(&mut list, 0 , 5);
        assert_eq!(is_sorted(&list), true);
    }
}
