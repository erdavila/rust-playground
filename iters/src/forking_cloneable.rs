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
        let mut shared_state = self.shared_state.borrow_mut();
        let buffer_index = self.state.next_item_number - shared_state.first_buffer_item_number;
        if buffer_index < shared_state.buffer.len() {
            todo!()
        }

        if let Some(next) = self.state.next {
            todo!()
        }

        if let Some(prev) = self.state.prev {
            todo!()
        }

        self.state.next_item_number += 1;
        shared_state.first_buffer_item_number += 1;

        if let Some(src) = shared_state.source.as_mut() {
            let next = src.next();
            if next.is_none() {
                shared_state.source = None;
            }

            next
        } else {
            None
        }
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
        if let Some(_) = self.state.prev {
            todo!();
        } else {
            if !self.shared_state.borrow().buffer.is_empty() {
                todo!()
            }
        }

        if let Some(_) = self.state.next {
            todo!();
        }
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
    use std::ops::Deref;

    use super::*;

    macro_rules! assert_forking_clones {
        ($iter:expr $(, $iters:expr)* $(,)?) => {{
            let iters = [& $iter, $(& $iters),*];

            let shared_state_rc = iters[0].shared_state.clone();
            let shared_state = shared_state_rc.borrow();

            assert_eq!(iters[0].state.next_item_number, shared_state.first_buffer_item_number);
            assert!(iters.first().unwrap().state.prev.is_none());
            assert!(iters.last().unwrap().state.next.is_none());

            for (i, iter) in iters.into_iter().enumerate() {
                assert!(Rc::ptr_eq(&iter.shared_state, &shared_state_rc));
                assert!(iter.state.next_item_number >= shared_state.first_buffer_item_number);
                assert!(iter.state.next_item_number <= shared_state.first_buffer_item_number + shared_state.buffer.len());

                if i > 0 {
                    let prev_iter_state = iters[i-1].state.deref();
                    assert!(prev_iter_state.next_item_number <= iter.state.next_item_number);
                    assert!(prev_iter_state.next.is_some_and(
                        |prev_next| std::ptr::eq(unsafe { prev_next.as_ref() }, iter.state.deref() )
                    ));
                    assert!(iter.state.prev.is_some_and(
                        |prev| std::ptr::eq(unsafe { prev.as_ref() }, prev_iter_state)
                    ));
                }
            }
        }};
    }

    #[test]
    fn no_clones() {
        let source = std::iter::successors(Some('A'), |c| char::from_u32(1 + *c as u32)).take(3);

        let mut iter = source.forking_cloneable();
        assert_forking_clones![iter];
        assert!(iter.shared_state.borrow().buffer.is_empty());

        let elem = iter.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter];
        assert!(iter.shared_state.borrow().buffer.is_empty());

        let elem = iter.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter];
        assert!(iter.shared_state.borrow().buffer.is_empty());

        let elem = iter.next();
        assert_eq!(elem, Some('C'));
        assert_forking_clones![iter];
        assert!(iter.shared_state.borrow().buffer.is_empty());

        let elem = iter.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter];
        assert!(iter.shared_state.borrow().buffer.is_empty());
        assert!(iter.shared_state.borrow().source.is_none());

        let elem = iter.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter];
    }
}
