use std::{cell::RefCell, collections::VecDeque, iter::FusedIterator, ptr::NonNull, rc::Rc};

pub trait ForkingCloneable: Iterator + Sized
where
    Self::Item: Clone,
{
    fn forking_cloneable(self) -> Iter<Self> {
        Iter::new(self)
    }
}

impl<I> ForkingCloneable for I
where
    I: Iterator,
    I::Item: Clone,
{
}

pub struct Iter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    // Having the state in a Box allows the iterator to be moved without moving the state.
    // The state is in a RefCell so that it can be mutated in Clone::clone.
    state: Box<RefCell<State>>,
    shared_state: Rc<RefCell<SharedState<I>>>,
}
impl<I> Iter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn new(source: I) -> Self {
        let state = State {
            next_item_number: 0,
            prev: None,
            next: None,
        };
        let shared_state = SharedState {
            source: Some(source),
            buffer: Buffer::new(),
        };

        Iter {
            state: Box::new(RefCell::new(state)),
            shared_state: Rc::new(RefCell::new(shared_state)),
        }
    }
}
impl<I> Iterator for Iter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut shared_state = self.shared_state.borrow_mut();
        let mut state = self.state.borrow_mut();

        let buffer_index = state.next_item_number - shared_state.buffer.first_item_number;
        if buffer_index < shared_state.buffer.len() {
            let elem = if state.prev.is_some()
                || state.next.is_some_and(|next| {
                    state.next_item_number == unsafe { next.as_ref().next_item_number }
                }) {
                shared_state.buffer.get(buffer_index)
            } else {
                shared_state.buffer.pop()
            };
            state.advance();
            Some(elem)
        } else {
            let elem = shared_state.next_elem();

            let is_single_iterator = state.prev.is_none() && state.next.is_none();
            if is_single_iterator {
                assert_eq!(
                    state.next_item_number,
                    shared_state.buffer.first_item_number
                );

                // Don't need to increment shared_state.buffer.first_item_number or state.next_item_number
            } else if let Some(elem) = &elem {
                shared_state.buffer.push(elem);
                state.advance();
            }

            elem
        }
    }
}
impl<I> Clone for Iter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        let mut state = self.state.borrow_mut();

        let clone_state = Box::new(RefCell::new(State {
            next_item_number: state.next_item_number,
            prev: None,
            next: None,
        }));

        state.insert_next(&mut clone_state.borrow_mut());

        let shared_state = Rc::clone(&self.shared_state);

        Iter {
            state: clone_state,
            shared_state,
        }
    }
}
impl<I> Drop for Iter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn drop(&mut self) {
        let mut state = self.state.borrow_mut();

        match (state.prev, state.next) {
            (None, None) => {}
            (None, Some(next)) => {
                state.unlink_next();

                let next_iterator_next_item_number = unsafe { next.as_ref().next_item_number };
                let mut shared_state = self.shared_state.borrow_mut();
                while shared_state.buffer.first_item_number < next_iterator_next_item_number {
                    shared_state.buffer.pop();
                }
            }
            (Some(_), None) => state.unlink_prev(),
            (Some(mut prev), Some(mut next)) => unsafe {
                State::link(prev.as_mut(), next.as_mut());
            },
        }
    }
}
impl<I> FusedIterator for Iter<I>
where
    I: Iterator,
    I::Item: Clone,
{
}

#[derive(Debug)]
struct State {
    next_item_number: usize,
    prev: Option<NonNull<State>>,
    next: Option<NonNull<State>>,
}
impl State {
    fn advance(&mut self) {
        self.next_item_number += 1;
        if let Some(mut next) = self.next
            && self.next_item_number > unsafe { next.as_ref().next_item_number }
        {
            if let Some(mut prev) = self.prev {
                unsafe {
                    Self::link(prev.as_mut(), next.as_mut());
                }
            } else {
                self.unlink_next();
            }

            while let Some(next_next) = unsafe { next.as_ref().next } {
                if self.next_item_number <= unsafe { next_next.as_ref().next_item_number } {
                    break;
                }
                next = next_next;
            }

            unsafe { next.as_mut().insert_next(self) };
        }
    }

    fn insert_next(&mut self, state: &mut State) {
        let next = self.next;
        State::link(self, state);
        state.link_next(next);
    }

    fn link_next(&mut self, next: Option<NonNull<State>>) {
        if let Some(mut next) = next {
            Self::link(self, unsafe { next.as_mut() });
        } else {
            self.next = None;
        }
    }

    fn unlink_prev(&mut self) {
        if let Some(mut prev) = self.prev {
            unsafe { prev.as_mut().next = None };
            self.prev = None;
        }
    }

    fn unlink_next(&mut self) {
        if let Some(mut next) = self.next {
            unsafe { next.as_mut().prev = None };
            self.next = None;
        }
    }

    fn link(prev: &mut State, next: &mut State) {
        let mut prev = NonNull::from(prev);
        let mut next = NonNull::from(next);

        unsafe {
            prev.as_mut().unlink_next();
            next.as_mut().unlink_prev();

            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev);
        }
    }
}

struct SharedState<I>
where
    I: Iterator,
{
    source: Option<I>,
    buffer: Buffer<I::Item>,
}
impl<I> SharedState<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn next_elem(&mut self) -> Option<I::Item> {
        if let Some(source) = &mut self.source {
            let elem = source.next();
            if elem.is_none() {
                self.source = None;
            }
            elem
        } else {
            None
        }
    }
}

struct Buffer<T> {
    elements: VecDeque<T>,
    first_item_number: usize,
}
impl<T> Buffer<T>
where
    T: Clone,
{
    fn new() -> Self {
        Buffer {
            elements: VecDeque::new(),
            first_item_number: 0,
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn get(&self, index: usize) -> T {
        self.elements[index].clone()
    }

    fn push(&mut self, elem: &T) {
        self.elements.push_back(elem.clone());
    }

    fn pop(&mut self) -> T {
        self.first_item_number += 1;
        unsafe { self.elements.pop_front().unwrap_unchecked() }
    }
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

            assert_eq!(iters[0].state.borrow().next_item_number, shared_state.buffer.first_item_number);
            assert!(iters.first().unwrap().state.borrow().prev.is_none());
            assert!(iters.last().unwrap().state.borrow().next.is_none());

            for (i, iter) in iters.into_iter().enumerate() {
                let iter_state = iter.state.borrow();

                assert!(Rc::ptr_eq(&iter.shared_state, &shared_state_rc));
                assert!(iter_state.next_item_number >= shared_state.buffer.first_item_number);
                assert!(iter_state.next_item_number <= shared_state.buffer.first_item_number + shared_state.buffer.len());

                if i > 0 {
                    let prev_iter_state = iters[i-1].state.borrow();
                    assert!(prev_iter_state.next_item_number <= iter_state.next_item_number);
                    assert!(prev_iter_state.next.is_some_and(
                        |prev_next| std::ptr::eq(unsafe { prev_next.as_ref() }, iter_state.deref() )
                    ));
                    assert!(iter_state.prev.is_some_and(
                        |prev| std::ptr::eq(unsafe { prev.as_ref() }, prev_iter_state.deref())
                    ));
                }
            }
        }};
    }

    fn get_iterator(count: usize) -> impl Iterator<Item = char> {
        let succ = |c: &char| char::from_u32(1 + *c as u32);
        std::iter::successors(Some('A'), succ).take(count)
    }

    #[test]
    fn no_clones() {
        let source = get_iterator(3);

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

    #[test]
    fn cloning() {
        let source = get_iterator(3);
        let mut iter1 = source.forking_cloneable();
        assert_forking_clones![iter1];

        let mut iter2 = iter1.clone();
        assert_forking_clones![iter1, iter2];

        let elem = iter1.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter2, iter1];

        let elem = iter1.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter2, iter1];

        let elem = iter2.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter2, iter1];

        let elem = iter2.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter2, iter1];

        let elem = iter2.next();
        assert_eq!(elem, Some('C'));
        assert_forking_clones![iter1, iter2];

        let elem = iter2.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter1, iter2];
        assert!(iter1.shared_state.borrow().source.is_none());

        let elem = iter1.next();
        assert_eq!(elem, Some('C'));
        assert_forking_clones![iter1, iter2];
        assert!(iter1.shared_state.borrow().buffer.is_empty());

        let elem = iter1.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter1, iter2];
    }

    #[test]
    fn single_iterator_consuming_from_buffer() {
        let source = get_iterator(2);
        let mut iter = source.forking_cloneable();
        iter.clone().for_each(drop); // Transfer remaining elements from source to buffer
        assert_forking_clones![iter];
        assert!(iter.shared_state.borrow().source.is_none());

        let elem = iter.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter];

        let elem = iter.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter];

        let elem = iter.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter];
    }

    #[test]
    fn several_iterators_consuming_from_buffer() {
        let source = get_iterator(2);
        let mut iter1 = source.forking_cloneable();
        iter1.clone().for_each(drop); // Transfer remaining elements from source to buffer
        assert_forking_clones![iter1];
        assert!(iter1.shared_state.borrow().source.is_none());

        let mut iter2 = iter1.clone();
        let mut iter3 = iter1.clone();
        assert_forking_clones![iter1, iter3, iter2];

        let elem = iter1.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter3, iter2, iter1];

        let elem = iter2.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter3, iter2, iter1];

        let elem = iter3.next();
        assert_eq!(elem, Some('A'));
        assert_forking_clones![iter3, iter2, iter1];

        let elem = iter3.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter2, iter1, iter3];

        let elem = iter2.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter1, iter2, iter3];

        let elem = iter1.next();
        assert_eq!(elem, Some('B'));
        assert_forking_clones![iter1, iter2, iter3];

        let elem = iter1.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter1, iter2, iter3];

        let elem = iter2.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter1, iter2, iter3];

        let elem = iter3.next();
        assert_eq!(elem, None);
        assert_forking_clones![iter1, iter2, iter3];
    }

    #[test]
    fn last_iterator_at_the_middle_of_the_buffer() {
        let source = get_iterator(5);
        let first = source.forking_cloneable();
        let mut last = first.clone();
        last.next();
        last.next();
        last.clone().for_each(drop); // Transfer remaining elements from source to buffer
        assert_forking_clones![first, last];

        let elem = last.next();
        assert_eq!(elem, Some('C'));
        assert_forking_clones![first, last];
    }

    #[test]
    fn drop_first_iterator_popping_from_buffer() {
        let source = get_iterator(2);
        let first = source.forking_cloneable();
        let mut second = first.clone();
        second.next();
        second.next();
        assert_forking_clones![first, second];
        assert_eq!(second.shared_state.borrow().buffer.len(), 2);
        assert!(first.state.borrow().next_item_number < second.state.borrow().next_item_number);

        drop(first);

        assert_forking_clones![second];
        assert!(second.shared_state.borrow().buffer.is_empty());
    }

    #[test]
    fn drop_first_iterator_keeping_buffer() {
        let source = get_iterator(2);
        let first = source.forking_cloneable();
        let second = first.clone();
        first.clone().for_each(drop); // Transfer remaining elements from source to buffer
        assert_forking_clones![first, second];
        assert_eq!(second.shared_state.borrow().buffer.len(), 2);
        assert!(first.state.borrow().next_item_number == second.state.borrow().next_item_number);
        assert!(first.state.borrow().prev.is_none());

        drop(first);

        assert_forking_clones![second];
        assert_eq!(second.shared_state.borrow().buffer.len(), 2);
    }

    #[test]
    fn drop_middle_iterator() {
        let source = get_iterator(2);
        let first = source.forking_cloneable();
        let mut middle = first.clone();
        middle.next();
        let mut last = middle.clone();
        last.next();
        assert_forking_clones![first, middle, last];

        drop(middle);

        assert_forking_clones![first, last];
    }
}
