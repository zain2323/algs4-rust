/*
Max and Min Index Priority Queue
 */

use std::slice::Iter;

struct Heap<T: Ord + Clone + Default> {
    // this hold indices
    pq: Vec<usize>,
    keys: Vec<Option<T>>,
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
            keys: vec![Some(Default::default()); n + 1],
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
        self.keys[k] = Some(key.clone());
        self.pq[self.n] = k;
        self.qp[k] = self.n as isize;
        self.swim(self.n, less);
    }

    // Change the item associated with k
    fn change(&mut self, k: usize, key: T, less: fn(T, T) -> bool) {
        // If the old key is smaller than the new key then sink method will reheapify the heap.
        // If the old key is greater than the new key then swim method will reheapify the heap.
        if self.contains(k) {
            let old_key = self.keys[k].clone();
            let key = Some(key);
            self.keys[k] = key.clone();
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
        if self.contains(k) {
            self.keys[k] = None;
            self.qp[k] = -1;
            self.sink(1, less);
            self.exch(1, self.n);
            self.n -= 1;
            self.sink(1, less);
        }
    }

    // Remove the minimal or maximal item and returns its index
    fn del(&mut self, less: fn(T, T) -> bool) -> usize {
        let index = self.index();
        self.exch(1, self.n);
        self.n -= 1;
        self.sink(1, less);
        let idx = self.pq[self.n + 1];
        self.keys[idx] = None;
        self.qp[idx] = -1;
        index
    }

    // Returns the minimal or maximal item
    fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Heap is empty")
        }
        self.keys[self.index()]
            .clone()
            .expect("Value does not exist")
    }

    // Returns the minimal or maximal item index
    fn index(&self) -> usize {
        self.pq[1]
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
    }

    fn swim(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while k > 1 {
            let key_1 = match self.keys[self.pq[k / 2]].clone() {
                Some(v) => v,
                None => continue,
            };

            let key_2 = match self.keys[self.pq[k]].clone() {
                Some(v) => v,
                None => continue,
            };
            if less(key_1, key_2) {
                self.exch(k / 2, k);
            }
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n {
                let mut key_1: T = Default::default();
                let mut key_2: T = Default::default();
                if let Some(v) = self.keys[self.pq[j]].clone() {
                    key_1 = v;
                }
                if let Some(v) = self.keys[self.pq[j + 1]].clone() {
                    key_2 = v;
                }
                if less(key_1, key_2) {
                    j += 1;
                }
            }
            let mut key_1: T = Default::default();
            let mut key_2: T = Default::default();
            if let Some(v) = self.keys[self.pq[k]].clone() {
                key_1 = v;
            }
            if let Some(v) = self.keys[self.pq[j]].clone() {
                key_2 = v;
            }
            if !less(key_1, key_2) {
                break;
            }
            self.exch(k, j);
            k = j
        }
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
}

// For max heap
fn less_max<T: Ord + Clone>(i: T, j: T) -> bool {
    i.lt(&j)
}

// For min heap
fn less_min<T: Ord + Clone>(i: T, j: T) -> bool {
    !i.lt(&j)
}

#[cfg(test)]
mod tests {
    use crate::fundamentals::indexed_pq;
    use std::cmp::max;

    #[test]
    fn insert() {
        let mut min_heap = indexed_pq::MinHeap::<&str>::new(5);
        let mut max_heap = indexed_pq::MaxHeap::<&str>::new(5);

        // Min heap
        assert_eq!(min_heap.is_empty(), true);
        min_heap.insert(3, "D");
        min_heap.insert(0, "F");
        min_heap.insert(1, "C");
        min_heap.insert(2, "E");
        min_heap.insert(4, "A");

        assert_eq!(min_heap.contains(1), true);
        assert_eq!(min_heap.contains(2), true);
        assert_eq!(min_heap.contains(3), true);
        assert_eq!(min_heap.contains(4), true);
        assert_eq!(min_heap.contains(0), true);
        assert_eq!(min_heap.contains(5), false);
        assert_eq!(min_heap.is_empty(), false);
        assert_eq!(min_heap.size(), 5);

        // Max heap
        assert_eq!(max_heap.is_empty(), true);
        max_heap.insert(3, "D");
        max_heap.insert(0, "F");
        max_heap.insert(1, "C");
        max_heap.insert(2, "E");
        max_heap.insert(4, "A");

        assert_eq!(max_heap.contains(1), true);
        assert_eq!(max_heap.contains(2), true);
        assert_eq!(max_heap.contains(3), true);
        assert_eq!(max_heap.contains(4), true);
        assert_eq!(max_heap.contains(0), true);
        assert_eq!(max_heap.contains(5), false);
        assert_eq!(max_heap.is_empty(), false);
        assert_eq!(max_heap.size(), 5);
    }

    #[test]
    fn deleting_extreme_values() {
        let mut min_heap = indexed_pq::MinHeap::<&str>::new(5);
        let mut max_heap = indexed_pq::MaxHeap::<&str>::new(5);

        // Min heap
        assert_eq!(min_heap.is_empty(), true);
        min_heap.insert(3, "D");
        min_heap.insert(0, "F");
        min_heap.insert(1, "C");
        min_heap.insert(2, "E");
        min_heap.insert(4, "A");

        // A will be deleted
        assert_eq!(min_heap.del_min(), 4);
        // C will be deleted
        assert_eq!(min_heap.del_min(), 1);
        // D will be deleted
        assert_eq!(min_heap.del_min(), 3);
        // E will be deleted
        assert_eq!(min_heap.del_min(), 2);
        // F will be deleted
        assert_eq!(min_heap.del_min(), 0);

        // Checking if the heap is empty after deleting all the keys.
        assert_eq!(min_heap.is_empty(), true);

        // Max heap
        assert_eq!(max_heap.is_empty(), true);
        max_heap.insert(3, "D");
        max_heap.insert(0, "F");
        max_heap.insert(1, "C");
        max_heap.insert(2, "E");
        max_heap.insert(4, "A");

        // F will be deleted
        assert_eq!(max_heap.del_max(), 0);
        // E will be deleted
        assert_eq!(max_heap.del_max(), 2);
        // D will be deleted
        assert_eq!(max_heap.del_max(), 3);
        // C will be deleted
        assert_eq!(max_heap.del_max(), 1);
        // A will be deleted
        assert_eq!(max_heap.del_max(), 4);

        // Checking if the heap is empty after deleting all the keys.
        assert_eq!(max_heap.is_empty(), true);
    }

    #[test]
    fn deleting_key_by_index() {
        let mut min_heap = indexed_pq::MinHeap::<&str>::new(5);
        let mut max_heap = indexed_pq::MaxHeap::<&str>::new(5);

        // Min heap
        assert_eq!(min_heap.is_empty(), true);
        min_heap.insert(3, "D");
        min_heap.insert(0, "F");
        min_heap.insert(1, "C");
        min_heap.insert(2, "E");
        min_heap.insert(4, "A");

        assert_eq!(min_heap.contains(1), true);
        assert_eq!(min_heap.contains(3), true);
        min_heap.delete(1);
        min_heap.delete(3);
        assert_eq!(min_heap.contains(1), false);
        assert_eq!(min_heap.contains(3), false);

        assert_eq!(min_heap.del_min(), 4);
        assert_eq!(min_heap.del_min(), 2);
        assert_eq!(min_heap.del_min(), 0);

        // Max heap
        assert_eq!(max_heap.is_empty(), true);
        max_heap.insert(3, "D");
        max_heap.insert(0, "F");
        max_heap.insert(1, "C");
        max_heap.insert(2, "E");
        max_heap.insert(4, "A");

        assert_eq!(max_heap.contains(1), true);
        assert_eq!(max_heap.contains(3), true);
        max_heap.delete(1);
        max_heap.delete(3);
        assert_eq!(max_heap.contains(1), false);
        assert_eq!(max_heap.contains(3), false);

        assert_eq!(max_heap.del_max(), 0);
        assert_eq!(max_heap.del_max(), 2);
        assert_eq!(max_heap.del_max(), 4);
    }

    #[test]
    fn changing_key_by_index() {
        let mut min_heap = indexed_pq::MinHeap::<&str>::new(5);
        let mut max_heap = indexed_pq::MaxHeap::<&str>::new(5);

        // Min heap
        assert_eq!(min_heap.is_empty(), true);
        min_heap.insert(3, "D");
        min_heap.insert(0, "F");
        min_heap.insert(1, "C");
        min_heap.insert(2, "E");
        min_heap.insert(4, "A");

        // Changing 'D' to 'B'
        min_heap.change(3, "B");
        // Changing 'A' to 'Z'
        min_heap.change(4, "Z");

        /*
        Now expected heap order is:
        B, C, E, F, Z
         */

        // B will be deleted
        assert_eq!(min_heap.del_min(), 3);
        // C will be deleted
        assert_eq!(min_heap.del_min(), 1);
        // E will be deleted
        assert_eq!(min_heap.del_min(), 2);
        // F will be deleted
        assert_eq!(min_heap.del_min(), 0);
        // Z will be deleted
        assert_eq!(min_heap.del_min(), 4);

        // Max heap
        assert_eq!(max_heap.is_empty(), true);
        max_heap.insert(3, "D");
        max_heap.insert(0, "F");
        max_heap.insert(1, "C");
        max_heap.insert(2, "E");
        max_heap.insert(4, "A");

        // Changing 'D' to 'B'
        max_heap.change(3, "B");
        // Changing 'A' to 'Z'
        max_heap.change(4, "Z");

        /*
        Now expected heap order is:
        Z, F, E, C, B
         */

        // Z will be deleted
        assert_eq!(max_heap.del_max(), 4);
        // F will be deleted
        assert_eq!(max_heap.del_max(), 0);
        // E will be deleted
        assert_eq!(max_heap.del_max(), 2);
        // C will be deleted
        assert_eq!(max_heap.del_max(), 1);
        // B will be deleted
        assert_eq!(max_heap.del_max(), 3);
    }
}
