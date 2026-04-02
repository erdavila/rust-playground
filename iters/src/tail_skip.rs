use std::collections::VecDeque;

pub trait TailSkip: Iterator + Sized {
    fn tail_skip(self, n: usize) -> TailSkipIter<Self> {
        TailSkipIter {
            n,
            iter: self,
            queue: VecDeque::with_capacity(n),
        }
    }
}

impl<I: Iterator> TailSkip for I {}

pub struct TailSkipIter<I: Iterator> {
    n: usize,
    iter: I,
    queue: VecDeque<I::Item>,
}

impl<I: Iterator> Iterator for TailSkipIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if self.queue.len() == self.n {
                let result = self.queue.pop_front();
                self.queue.push_back(item);
                return result;
            }
            self.queue.push_back(item);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let iter = (1..=10).tail_skip(3);
        assert!(iter.eq(1..=7));

        let iter = (1..=3).tail_skip(3);
        assert!(iter.eq([]));

        let iter = (1..=3).tail_skip(5);
        assert!(iter.eq([]));
    }
}
