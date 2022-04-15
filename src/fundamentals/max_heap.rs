/*
Max Heap Priority Queue
 */

struct MaxPQ<T: Ord + Copy> {
    pq: Vec<T>,
    n: usize,
}

impl<T: Ord + Copy> MaxPQ<T> {
    pub fn new() -> MaxPQ<T> {
        MaxPQ {
            pq: Vec::new(),
            n: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn insert(&mut self, key: T) {
        if self.is_empty() {
            self.pq.insert(0, key);
        }
        self.n += 1;
        self.pq.insert(self.n, key);
        self.swim(self.n);
    }

    pub fn del_max(&mut self) -> T {
        let max = self.peek();
        self.exch(1, self.pq.len() - 1);
        self.pq.remove(self.pq.len() - 1);
        self.n -= 1;
        self.sink(1);
        max
    }

    pub fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Heap is empty")
        }
        self.pq[1]
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.exch((k / 2) as usize, k as usize);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.less(j, j + 1) {
                j += 1;
            }
            if !self.less(k, j) {
                break;
            }
            self.exch(k as usize, j as usize);
            k = j
        }
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.pq[i].lt(&self.pq[j])
    }
}

#[cfg(test)]
mod tests {
    use crate::fundamentals::max_heap;

    #[test]
    fn empty_heap() {
        let heap = max_heap::MaxPQ::<i32>::new();
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn heap_size_one() {
        let mut heap = max_heap::MaxPQ::<i32>::new();
        heap.insert(10);
        assert_eq!(heap.size(), 1);
    }

    #[test]
    fn heap_size_two() {
        let mut heap = max_heap::MaxPQ::<i32>::new();
        heap.insert(10);
        heap.insert(11);
        assert_eq!(heap.size(), 2);
    }

    #[test]
    fn max() {
        let mut heap = max_heap::MaxPQ::<i32>::new();
        heap.insert(10);
        heap.insert(11);
        heap.insert(13);
        heap.insert(9);
        heap.insert(12);
        heap.insert(1);
        assert_eq!(heap.del_max(), 13);
        assert_eq!(heap.del_max(), 12);
        assert_eq!(heap.del_max(), 11);
        assert_eq!(heap.del_max(), 10);
        assert_eq!(heap.del_max(), 9);
        assert_eq!(heap.peek(), 1);
        assert_eq!(heap.del_max(), 1);
    }
}
