/*
Bag implementation using Vector
*/

#[derive(Clone)]
pub struct Bag<T: Copy> {
    item: Vec<T>,
    n: usize,
}

impl<T: Copy> Bag<T> {
    pub fn new() -> Bag<T> {
        Bag {
            item: Vec::new(),
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn add(&mut self, item: T) {
        self.item.push(item);
        self.n += 1;
    }

    pub fn iter(&self) -> BagIterator<T> {
        BagIterator {
            bag: self.clone(),
            index: self.n.clone(),
        }
    }
}

pub struct BagIterator<T: Copy> {
    bag: Bag<T>,
    index: usize,
}

impl<T: Copy> Iterator for BagIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= 0 {
            return None;
        }
        self.index -= 1;
        let item = self.bag.item[self.index];
        return Some(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bag() {
        let bag: Bag<i32> = Bag::new();
        assert_eq!(bag.is_empty(), true);
    }

    #[test]
    fn add() {
        let mut bag: Bag<i32> = Bag::new();
        bag.add(1);
        bag.add(2);
        bag.add(3);
        assert_eq!(bag.size(), 3);
        assert_eq!(bag.is_empty(), false);
    }

    #[test]
    fn iterating_using_iter() {
        let mut bag: Bag<i32> = Bag::new();
        for i in 0..5 {
            bag.add(i);
        }

        let mut counter = 4;
        for item in bag.iter() {
            assert_eq!(item, counter);
            counter -= 1;
        }
    }
}
