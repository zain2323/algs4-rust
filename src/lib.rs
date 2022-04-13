pub mod sorting;
pub mod fundamentals;
pub mod io;

#[cfg(test)]
mod tests {
    use crate::sorting::insertion;
    use crate::fundamentals::stack::Stack;

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
}
