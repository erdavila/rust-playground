use std::{cell::RefCell, collections::VecDeque, iter::FusedIterator, ptr::NonNull, rc::Rc};

pub trait ForkingCloneable: Iterator + Sized
where
    Self::Item: Clone,
{
    fn forking_cloneable(self) -> ForkingCloneableIter<Self> {
        ForkingCloneableIter::new(self)
    }
}

impl<I> ForkingCloneable for I
where
    I: Iterator,
    I::Item: Clone,
{
}

pub struct ForkingCloneableIter<I>
where
    I: Iterator,
{
    // Having the state in a Box allows the iterator to be moved without moving the state
    state: Box<State>,
    shared_state: Rc<RefCell<SharedState<I>>>,
}
impl<I> ForkingCloneableIter<I>
where
    I: Iterator,
{
    fn new(source: I) -> Self {
        let state = State {
            next_item_number: 0,
            prev: None,
            next: None,
        };
        let shared_state = SharedState {
            source: Some(source),
            buffer: VecDeque::new(),
            first_buffer_item_number: 0,
        };

        ForkingCloneableIter {
            state: Box::new(state),
            shared_state: Rc::new(RefCell::new(shared_state)),
        }
    }
}
impl<I> Iterator for ForkingCloneableIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
impl<I> Clone for ForkingCloneableIter<I>
where
    I: Iterator,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<I> Drop for ForkingCloneableIter<I>
where
    I: Iterator,
{
    fn drop(&mut self) {
        todo!()
    }
}
impl<I> FusedIterator for ForkingCloneableIter<I> where I: Iterator {}

struct State {
    next_item_number: usize,
    prev: Option<NonNull<State>>,
    next: Option<NonNull<State>>,
}

struct SharedState<I>
where
    I: Iterator,
{
    source: Option<I>,
    buffer: VecDeque<I::Item>,
    first_buffer_item_number: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
