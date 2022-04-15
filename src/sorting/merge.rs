/*
Sorts the array entries using the Merge Sort Algorithm.
 */

// use crate::sorting::utils::{less, exch};

// Merges the two sorted arrays
fn merge<T: Ord + Copy>(arr: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    //     Copy a[lo..hi] to aux[lo..hi]
    for k in lo..=hi {
        aux[k] = arr[k];
    }

    let mut i = lo;
    let mut j = mid + 1;
    for k in lo..=hi {
        if i > mid {
            arr[k] = aux[j];
            j += 1;
        } else if j > hi {
            arr[k] = aux[i];
            i += 1;
        } else if less(aux[j], aux[i]) {
            arr[k] = aux[j];
            j += 1;
        } else {
            arr[k] = aux[i];
            i += 1;
        }
    }
}

pub fn sort<T: Ord + Copy>(arr: &mut Vec<T>) {
    let mut aux: Vec<T> = vec![arr[0]; arr.len()];
    merge_sort(arr, &mut aux, 0, arr.len() - 1);
}

fn merge_sort<T: Ord + Copy>(arr: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    merge_sort(arr, aux, lo, mid);
    merge_sort(arr, aux, mid + 1, hi);
    merge(arr, aux, lo, mid, hi);
}

pub fn less<T: Ord>(v: T, w: T) -> bool {
    v.lt(&w)
}

// exchange arr[i] and arr[j]
pub fn exch<T: Ord>(arr: &mut Vec<T>, i: usize, j: usize) {
    arr.swap(i, j);
}

// checks if the array is sorted?
pub fn is_sorted<T: Ord + Copy>(list: &Vec<T>) -> bool {
    for i in 1..list.len() {
        if less(list[i], list[i - 1]) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::sorting::merge::sort;
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
