/*
Helper Methods
 */

// is v < w
pub fn less<T: PartialOrd>(v: &T , w: &T) -> bool {
    v.lt(w)
}

// exchange arr[i] and arr[j]
pub fn exch<T: PartialOrd + Clone>(arr: &mut Vec<T>, i:usize, j:usize) {
    let temp = arr[i].clone();
    arr[i] = arr[j].clone();
    arr[j] = temp;
}

// checks if the array is sorted?
pub fn is_sorted<T: PartialOrd>(list: &Vec<T>) -> bool {
    for i in 1..list.len() {
        if less(&list[i],&list[i-1]) {
            return false;
        }
    }
    return true;
}