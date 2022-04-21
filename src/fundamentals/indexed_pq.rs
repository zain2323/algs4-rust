/*
Max and Min Index Priority Queue
 */

struct Heap<T: Ord + Clone + Default> {
    // this hold indices
    pq: Vec<usize>,
    keys: Vec<T>,
    qp: Vec<isize>,
    n: usize,
}

pub struct MaxHeap<T: Ord + Clone + Default> {
    heap: Heap<T>,
}

pub struct MinHeap<T: Ord + Clone + Default> {
    heap: Heap<T>,
}

impl<T: Ord + Clone + Default> Heap<T> {
    fn new(n: usize) -> Heap<T> {
        Heap {
            pq: vec![0; n + 1],
            keys: vec![Default::default(); n + 1],
            qp: vec![-1; n + 1],
            n: 0,
        }
    }

    // Is the Priority Queue empty?
    fn is_empty(&self) -> bool {
        self.n == 0
    }

    // Number of items in the Priority Queue
    fn size(&self) -> usize {
        self.n
    }

    // Insert item and associate it with k
    fn insert(&mut self, k: usize, key: T, less: fn(T, T) -> bool) {
        self.n += 1;
        self.keys[k] = key.clone();
        self.pq[self.n] = k;
        self.qp[k] = self.n as isize;
        self.swim(self.n, less);
    }

    // Change the item associated with k
    fn change(&mut self, k: usize, key: T, less: fn(T, T) -> bool) {
        self.inverse();
        // If the old key is smaller than the new key then sink method will reheapify the heap.
        // If the old key is greater than the new key then swim method will reheapify the heap.
        if self.contains(k) {
            let idx = self.get_pos(k) + 1;
            let old_key = self.keys[idx].clone();
            self.keys[idx] = key.clone();
            if old_key < key {
                self.sink(1, less);
            } else if old_key > key {
                self.swim(self.size(), less)
            }
        }
    }

    // Is k associated with some item
    fn contains(&mut self, k: usize) -> bool {
        self.qp[k] != -1
    }

    // Remove k and its associated item
    fn delete(&mut self, k: usize, less: fn(T, T) -> bool) {
        self.inverse();
        if self.contains(k) {
            let idx = self.get_pos(k) + 1;
            self.keys.remove(idx);
            self.pq.remove(idx);
            self.qp[k] = -1;
            self.n -= 1;
            self.sink(1, less)
        }
    }

    // Remove the minimal or maximal item and returns its index
    fn del(&mut self, less: fn(T, T) -> bool) -> usize {
        let index = self.index();
        self.exch(1, self.pq.len() - 1);
        self.n -= 1;
        self.sink(1, less);
        let idx = self.pq[self.n + 1];
        self.keys[idx] = Default::default();
        self.qp[idx] = -1;
        index
    }

    // Returns the minimal or maximal item
    fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Heap is empty")
        }
        self.keys[self.index()].clone()
    }

    // Returns the minimal or maximal item index
    fn index(&self) -> usize {
        self.pq[1]
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
    }

    fn swim(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while k > 1
            && less(
                self.keys[self.pq[k / 2]].clone(),
                self.keys[self.pq[k]].clone(),
            )
        {
            self.exch(k / 2, k);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n
                && less(
                    self.keys[self.pq[j]].clone(),
                    self.keys[self.pq[j + 1]].clone(),
                )
            {
                j += 1;
            }
            if !less(self.keys[self.pq[k]].clone(), self.keys[self.pq[j]].clone()) {
                break;
            }
            self.exch(k, j);
            k = j
        }
    }

    fn set_pos(&mut self, k: usize) {
        let idx = self.index_of(k, &self.pq);
        match idx {
            Some(v) => self.qp[k] = v as isize,
            None => self.qp[k] = -1,
        }
    }

    fn inverse(&mut self) {
        for i in 0..self.pq.len() {
            self.set_pos(i)
        }
    }

    fn index_of(&self, element: usize, arr: &Vec<usize>) -> Option<usize> {
        for i in 1..arr.len() {
            if arr[i] == element {
                return Some(i);
            }
        }
        return None;
    }

    fn get_pos(&self, k: usize) -> usize {
        (self.qp[k] - 1) as usize
    }

    fn iter(&mut self) -> Vec<T> {
        self.keys[1..].to_vec()
    }
}

impl<T: Ord + Clone + Default> MaxHeap<T> {
    pub fn new(n: usize) -> MaxHeap<T> {
        MaxHeap { heap: Heap::new(n) }
    }

    pub fn insert(&mut self, k: usize, key: T) {
        self.heap.insert(k, key, less_max);
    }

    pub fn change(&mut self, k: usize, key: T) {
        self.heap.change(k, key, less_max);
    }

    pub fn contains(&mut self, k: usize) -> bool {
        self.heap.contains(k)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }

    pub fn max(&self) -> T {
        self.heap.peek()
    }

    pub fn max_index(&self) -> usize {
        self.heap.index()
    }

    pub fn del_max(&mut self) -> usize {
        self.heap.del(less_min)
    }

    pub fn delete(&mut self, k: usize) {
        self.heap.delete(k, less_min);
    }

    pub fn iter(&mut self) -> Vec<T> {
        self.heap.iter()
    }
}

impl<T: Ord + Clone + Default> MinHeap<T> {
    pub fn new(n: usize) -> MinHeap<T> {
        MinHeap { heap: Heap::new(n) }
    }

    pub fn insert(&mut self, k: usize, key: T) {
        self.heap.insert(k, key, less_min);
    }

    pub fn change(&mut self, k: usize, key: T) {
        self.heap.change(k, key, less_min);
    }

    pub fn contains(&mut self, k: usize) -> bool {
        self.heap.contains(k)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }

    pub fn min(&self) -> T {
        self.heap.peek()
    }

    pub fn min_index(&self) -> usize {
        self.heap.index()
    }

    pub fn del_min(&mut self) -> usize {
        self.heap.del(less_min)
    }

    pub fn delete(&mut self, k: usize) {
        self.heap.delete(k, less_min);
    }

    pub fn iter(&mut self) -> Vec<T> {
        self.heap.iter()
    }
}

fn less_max<T: Ord + Clone>(i: T, j: T) -> bool {
    i.lt(&j)
}

fn less_min<T: Ord + Clone>(i: T, j: T) -> bool {
    !i.lt(&j)
}

#[cfg(test)]
mod tests {
    use crate::fundamentals::indexed_pq;

    #[test]
    fn min_heap() {
        let mut heap = indexed_pq::MinHeap::<&str>::new(5);
        assert_eq!(heap.is_empty(), true);
        heap.insert(3, "D");
        heap.insert(0, "F");
        heap.insert(1, "C");
        heap.insert(2, "E");
        heap.insert(4, "A");

        assert_eq!(heap.contains(1), true);
        assert_eq!(heap.contains(2), true);
        assert_eq!(heap.contains(3), true);
        assert_eq!(heap.contains(4), true);
        assert_eq!(heap.contains(0), true);
        assert_eq!(heap.contains(5), false);
        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.size(), 5);
    }

    #[test]
    fn changing_key() {
        let mut heap = indexed_pq::MinHeap::<&str>::new(5);
        assert_eq!(heap.is_empty(), true);
        heap.insert(3, "D");
        heap.insert(0, "F");
        heap.insert(1, "C");
        heap.insert(2, "E");
        heap.insert(4, "A");
        // Changing Key 'D' to 'B' and new key is smaller
        // heap.change(3, "B");

        // A will be deleted
        assert_eq!(heap.del_min(), 4);
        // B will be deleted
        // assert_eq!(heap.del_min(), 1);
        // C will be deleted
        // assert_eq!(heap.del_min(), 3);
        // E will be deleted
        // assert_eq!(heap.del_min(), 2);
        // F will be deleted
        // assert_eq!(heap.del_min(), 0);

        // Checking if the heap is empty after deleting all the keys.
        // assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn deleting_key_by_index() {
        let mut heap = indexed_pq::MinHeap::<&str>::new(5);
        assert_eq!(heap.is_empty(), true);
        heap.insert(3, "D");
        heap.insert(0, "F");
        heap.insert(1, "C");
        heap.insert(2, "E");
        heap.insert(4, "A");

        heap.delete(1);
        assert_eq!(heap.contains(1), false);
        assert_eq!(heap.contains(3), true);
        assert_eq!(heap.contains(10), false);

        assert_eq!(heap.min(), "A");
        assert_eq!(heap.min_index(), 4);
    }
}
