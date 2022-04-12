pub mod sorting;

#[cfg(test)]
mod tests {
    use crate::sorting::insertion;
    #[test]
    fn it_works() {
        let mut arr = vec![10, 21, 3, 19, 0, 1];
        assert_eq!(insertion::sort(&mut arr), arr.sort());
    }
}
