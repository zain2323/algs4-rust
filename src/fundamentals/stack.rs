#[derive(Clone)]
pub struct Stack<T: Copy> {
    item: Vec<T>,
    n: usize,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            item: Vec::new(),
            n: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, item: T) {
        self.item.push(item);
        self.n += 1;
    }

    pub fn pop(&mut self) -> T {
        if self.is_empty() {
            panic!("Stack Underflow");
        }
        self.n -= 1;
        match self.item.pop() {
            Some(item) => item,
            None => panic!("Stack Underflow"),
        }
    }

    pub fn peek(&self) -> T {
        if self.is_empty() {
            panic!("Stack Underflow");
        }
        self.item[self.n]
    }

    pub fn iter(&self) -> StackIterator<T> {
        StackIterator {
            stack: self.clone(),
            index: self.n.clone(),
        }
    }
}

pub struct StackIterator<T: Copy> {
    stack: Stack<T>,
    index: usize,
}

impl<T: Copy> Iterator for StackIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= 0 {
            return None;
        }
        self.index -= 1;
        let item = self.stack.item[self.index];
        return Some(item);
    }
}

impl<T: Copy> IntoIterator for &Stack<T> {
    type Item = T;
    type IntoIter = StackIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator {
            stack: self.clone(),
            index: self.n.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fundamentals::stack::Stack;

    #[test]
    fn empty_stack() {
        let testing_stack: Stack<i32> = Stack::new();
        assert_eq!(testing_stack.is_empty(), true);
    }

    #[test]
    fn size_one() {
        let mut testing_stack: Stack<i32> = Stack::new();
        testing_stack.push(1);
        assert_eq!(testing_stack.size(), 1);
    }

    #[test]
    fn push() {
        let mut testing_stack: Stack<i32> = Stack::new();
        for i in 1..6 {
            testing_stack.push(i);
        }
        assert_eq!(testing_stack.size(), 5);
    }

    #[test]
    fn pop() {
        let mut testing_stack: Stack<i32> = Stack::new();
        for i in 1..6 {
            testing_stack.push(i);
        }

        let pop_item = testing_stack.pop();
        assert_eq!(pop_item, 5);
        assert_eq!(testing_stack.size(), 4);
    }

    #[test]
    fn iterating_using_iter() {
        let mut testing_stack: Stack<i32> = Stack::new();
        for i in 0..5 {
            testing_stack.push(i);
        }

        let mut counter = 4;
        for item in testing_stack.iter() {
            assert_eq!(item, counter);
            counter -= 1;
        }
    }

    #[test]
    fn iterating_using_into_iter() {
        let mut testing_stack: Stack<i32> = Stack::new();
        for i in 0..5 {
            testing_stack.push(i);
        }

        let mut counter = 4;
        for item in &testing_stack {
            assert_eq!(item, counter);
            counter -= 1;
        }
    }
}
