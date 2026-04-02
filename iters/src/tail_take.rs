use std::collections::VecDeque;

pub trait TailTake: Iterator + Sized {
    fn tail_take(self, n: usize) -> TailTakeIter<Self> {
        TailTakeIter {
            n,
            iter: self,
            queue: VecDeque::with_capacity(n),
        }
    }
}

impl<I: Iterator> TailTake for I {}

pub struct TailTakeIter<I: Iterator> {
    n: usize,
    iter: I,
    queue: VecDeque<I::Item>,
}

impl<I: Iterator> Iterator for TailTakeIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if self.queue.len() == self.n {
                self.queue.pop_front();
            }
            self.queue.push_back(item);
        }
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let iter = (1..=10).tail_take(3);
        assert!(iter.eq(8..=10));

        let iter = (1..=3).tail_take(3);
        assert!(iter.eq(1..=3));

        let iter = (1..=3).tail_take(5);
        assert!(iter.eq(1..=3));
    }
}
