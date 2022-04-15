/*
Helper Methods
 */

pub fn less<T: Ord>(v: T , w: T) -> bool {
    v.lt(&w)
}

// exchange arr[i] and arr[j]
pub fn exch<T: Ord>(arr: &mut Vec<T>, i:usize, j:usize) {
    arr.swap(i, j);
}

// checks if the array is sorted?
pub fn is_sorted<T: Ord + Copy>(list: &Vec<T>) -> bool {
    for i in 1..list.len() {
        if less(list[i],list[i-1]) {
            return false;
        }
    }
    return true;
}