pub mod fundamentals;
pub mod io;
pub mod sorting;

#[cfg(test)]
mod tests {
    use crate::fundamentals;
    use crate::fundamentals::stack::Stack;
    use crate::sorting::insertion;

    #[test]
    fn testing_sorting() {
        let mut arr = vec![10, 21, 3, 19, 0, 1];
        assert_eq!(insertion::sort(&mut arr), arr.sort());
    }

    #[test]
    fn testing_fundamentals() {
        let mut stack = Stack::new();
        for i in 1..=10 {
            stack.push(i);
        }

        let pop_item = stack.pop();
        assert_eq!(pop_item, 10);
        assert_eq!(stack.size(), 9);

        let mut counter = 9;
        for item in stack.iter() {
            assert_eq!(item, counter);
            counter -= 1;
        }
    }

    #[test]
    fn testing_heap() {
        let mut heap = fundamentals::heap::MaxHeap::<i32>::new();
        assert_eq!(heap.is_empty(), true);
        heap.insert(1);
        heap.insert(2);
        assert_eq!(heap.del_max(), 2);
        assert_eq!(heap.del_max(), 1);
    }
}
