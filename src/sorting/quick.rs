use std::fmt::Debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::sorting::utils::{exch, less};

pub fn sort<T: PartialOrd + Clone + Copy + Debug>(arr: &mut Vec<T>) {
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);
    quick_sort(arr, 0, arr.len()-1);
}

fn quick_sort<T: PartialOrd + Clone + Copy + Debug>(arr: &mut Vec<T>, lo: usize, hi: usize) {
    if hi <= lo {return;}
    let j = partition(arr, lo, hi);
    if j > 0 {
        quick_sort(arr, lo, j-1);
    }
    else {
        quick_sort(arr, lo, j);
    }

    quick_sort(arr, j+1, hi);
}

fn partition<T: PartialOrd + Clone + Copy + Debug>(arr: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi + 1;
    let v = arr[lo];
    loop {
        i += 1;
        while less(&arr[i], &v) {
            if i == hi {break}
            i += 1;
        }
        j -= 1;
        while less(&v, &arr[j]) {
            if j == lo {break}
            j -= 1;
        }
        if i >= j {break}
        exch(arr, i, j);
    }
    exch(arr, lo, j);
    j
}

#[cfg(test)]
mod tests {
    use crate::sorting::quick::sort;
    use crate::sorting::utils::is_sorted;

    #[test]
    fn test1_sort_random() {
        let mut list = vec![0, 100, 21, 24, 199, 221, 1021];
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
