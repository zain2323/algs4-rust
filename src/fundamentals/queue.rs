#[derive(Clone)]
pub struct Queue<T: Copy> {
    item: Vec<T>,
    n: usize
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            item: Vec::new(),
            n: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn enqueue(&mut self, item: T) {
        self.item.push(item);
        self.n += 1;
    }

    pub fn dequeue(&mut self) -> T {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.n -= 1;
        self.item.remove(0)
    }

    pub fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Queue is empty");
        }
        self.item[0]
    }

    pub fn iter(&self) -> QueueIterator<T> {
        QueueIterator {
            queue: self.clone(),
            index: 0 as usize
        }
    }
}

pub struct QueueIterator<T: Copy> {
    queue: Queue<T>,
    index: usize
}

impl<T: Copy> Iterator for QueueIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.size() {
            return None;
        }

        let item = Some(self.queue.item[self.index]);
        self.index += 1;
        item
    }
}

impl<T: Copy> IntoIterator for &Queue<T> {
    type Item = T;
    type IntoIter = QueueIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        QueueIterator {
            queue: self.clone(),
            index: self.n.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fundamentals::queue::{Queue, QueueIterator};

    #[test]
    fn empty_queue() {
        let mut testing_queue: Queue<i32> = Queue::new();
        assert_eq!(testing_queue.is_empty(), true);
    }

    #[test]
    fn size_one() {
        let mut testing_queue: Queue<i32> = Queue::new();
        testing_queue.enqueue(1);
        assert_eq!(testing_queue.size(), 1);
    }

    #[test]
    fn enqueue() {
        let mut testing_queue: Queue<i32> = Queue::new();
        for i in 1..6 {
            testing_queue.enqueue(i);
        }
        assert_eq!(testing_queue.size(), 5);
    }

    #[test]
    fn dequeue() {
        let mut testing_queue: Queue<i32> = Queue::new();
        for i in 1..6 {
            testing_queue.enqueue(i);
        }

        let dequeue_item = testing_queue.dequeue();
        assert_eq!(dequeue_item, 1);
        assert_eq!(testing_queue.size(), 4);
    }

    #[test]
    fn iterating_using_iter() {
        let mut testing_queue: Queue<i32> = Queue::new();
        for i in 0..5 {
            testing_queue.enqueue(i);
        }

        let mut counter = 0;
        for item in testing_queue.iter() {
            println!("{}", item);
            assert_eq!(item, counter);
            counter += 1;
        }
    }

    #[test]
    fn iterating_using_into_iter() {
        let mut testing_queue: Queue<i32> = Queue::new();
        for i in 0..5 {
            testing_queue.enqueue(i);
        }

        let mut counter = 0;
        for item in &testing_queue {
            assert_eq!(item, counter);
            counter += 1;
        }
    }
}