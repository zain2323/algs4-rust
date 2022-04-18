/*
Max and Min Index Priority Queue
 */

use std::io::stdout;

struct Heap<T: Ord + Copy> {
    // this hold indices
    pq: Vec<usize>,
    keys: Vec<T>,
    qp: Vec<isize>,
    n: usize
}

// pub struct MaxHeap<T: Ord + Copy> {
//     heap: Heap<T>,
// }

pub struct MinHeap<T: Ord + Copy> {
    heap: Heap<T>,
}

impl<T: Ord + Copy> Heap<T> {
    fn new() -> Heap<T> {
        Heap {
            pq: Vec::new(),
            keys: Vec::new(),
            qp: Vec::new(),
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
        if self.is_empty() {
            self.keys.insert(0, key);
            self.pq.insert(0, usize::MAX);
            self.qp.insert(0, -2);
        }
        self.n += 1;
        self.keys.insert(self.n, key);
        self.pq.insert(self.n, k);
        self.qp.insert(self.n, k as isize);
        // self.set_pos(k);
        self.swim(self.n, less);
    }

    // Change the item associated with k
    fn change(&mut self, k: usize, key: T) {
        self.inverse();
        // if self.contains(k) {
        //     let idx = self.get_pos(k);
        //     let pos = self.pq[idx as usize];
        //     self.keys[pos] = key;
        // }

    }

    // Is k associated with some item
    fn contains(&self, k: usize) -> bool {
        self.get_pos(k) != -1
    }

    // Remove k and its associated item
    fn delete(&mut self, k: usize) {
        todo!()
    }

    // Remove the minimal or maximal item and returns its index
    fn del(&mut self, less: fn(T, T) -> bool) -> usize {
        let index = self.index();
        self.exch(1, self.pq.len() - 1);
        self.keys.remove(self.keys.len()-1);
        self.pq.remove(self.pq.len() - 1);
        self.n -= 1;
        self.sink(1, less);
        index
    }

    // Returns the minimal or maximal item
    fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Heap is empty")
        }
        self.keys[1]
    }

    // Returns the minimal or maximal item index
    fn index(&self) -> usize {
        self.pq[1]
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.keys.swap(i, j);
        self.pq.swap(i, j);
    }

    fn swim(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while k > 1 && less(self.keys[k / 2], self.keys[k]) {
            self.exch(k/2, k);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize, less: fn(T, T) -> bool) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && less(self.keys[j], self.keys[j + 1]) {
                j += 1;
            }
            if !less(self.keys[k], self.keys[j]) {
                break;
            }
            self.exch(k, j);
            k = j
        }
    }

    fn set_pos(&mut self, k: usize) {
        // if self.qp.len() <= k {
        //     for _ in 0..k+2 - self.qp.len() {
        //         self.qp.push(-1);
        //     }
        // }

        let idx = self.index_of(k, &self.pq);
        match idx {
            Some(v) => self.qp[k] = v as isize ,
            None => self.qp[k] = -1
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
                return Some(i)
            }
        }
        return None;
    }

    fn get_pos(&self, k: usize) -> isize {
        self.qp[k] - 1
    }

    // fn iter(&mut self) -> Vec<T> {
    //     self.pq[1..].to_vec()
    // }
}

// impl<T: Ord + Copy> MaxHeap<T> {
//     pub fn new() -> MaxHeap<T> {
//         MaxHeap { heap: Heap::new() }
//     }
//
//     pub fn insert(&mut self, key: T) {
//         self.heap.insert(key, less_max);
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.heap.is_empty()
//     }
//
//     pub fn size(&self) -> usize {
//         self.heap.size()
//     }
//
//     pub fn peek(&self) -> T {
//         self.heap.peek()
//     }
//
//     pub fn del_max(&mut self) -> T {
//         self.heap.del(less_max)
//     }
//
//     pub fn iter(&mut self) -> Vec<T> {
//         self.heap.iter()
//     }
// }

impl<T: Ord + Copy> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { heap: Heap::new() }
    }

    pub fn insert(&mut self, k: usize, key: T) {
        self.heap.insert(k, key, less_min);
    }

    pub fn change(&mut self, k: usize, key: T) {
        self.heap.change(k, key);
    }

    pub fn contains(&self, k: usize) -> bool {
        todo!()
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
        self.heap.delete(k);
    }
    // pub fn iter(&mut self) -> Vec<T> {
    //     self.heap.iter()
    // }

    pub fn get_pos(&self, k: usize) -> isize {
        self.heap.get_pos(k)
    }
}

fn less_max<T: Ord + Copy>(i: T, j: T) -> bool {
    i.lt(&j)
}

fn less_min<T: Ord + Copy>(i: T, j: T) -> bool {
    !i.lt(&j)
}

#[cfg(test)]
mod tests {
    use crate::fundamentals::index_min_pq;

    #[test]
    fn min_heap() {
        let mut heap = index_min_pq::MinHeap::<&str>::new();
        assert_eq!(heap.is_empty(), true);
        heap.insert(3, "D");
        heap.insert(0, "F");
        heap.insert(1, "C");
        heap.insert(2, "E");
        heap.insert(4, "A");
        heap.change(3, "B");
        let mut pos =  heap.get_pos(0);
        println!("{}", pos);
        pos = heap.get_pos(1);
        println!("{}", pos);
        pos = heap.get_pos(2);
        println!("{}", pos);
        pos = heap.get_pos(3);
        println!("{}", pos);
        pos = heap.get_pos(4);
        println!("{}", pos);
        // assert_eq!(heap.is_empty(), false);
        // assert_eq!(heap.size(), 5);
        // assert_eq!(heap.del_min(), 4);
        // assert_eq!(heap.del_min(), 1);
        // assert_eq!(heap.del_min(), 3);
        // assert_eq!(heap.del_min(), 2);
        // assert_eq!(heap.del_min(), 0);
        // assert_eq!(heap.is_empty(), true);
    }
}
