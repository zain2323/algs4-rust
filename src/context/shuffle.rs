// Implements various shuffling algorithms

use rand::{thread_rng, Rng};

trait Shuffler<T>
where
    T: Ord,
{
    fn shuffle(&mut self, slice: &mut [T]);
}

pub struct KnuthShuffle;
impl<T> Shuffler<T> for KnuthShuffle
where
    T: Ord,
{
    fn shuffle(&mut self, slice: &mut [T]) {
        let n = slice.len();
        for i in 0..n {
            let random_num: usize = thread_rng().gen_range(0..i + 1);
            slice.swap(i, random_num);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_shuffles() {
        let mut slice = vec![1, 2, 3, 4, 5];
        KnuthShuffle.shuffle(&mut slice);
        assert_ne!(slice, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_shuffles_with_duplicates() {
        let mut slice = vec![100, 21, 12, 41, 54, 90, 13, 12];
        KnuthShuffle.shuffle(&mut slice);
        assert_ne!(slice, &[100, 21, 12, 41, 54, 90, 13, 12]);
    }
}
