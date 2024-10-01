use std::{cell::RefCell, collections::VecDeque, fmt::Debug, iter::FusedIterator, rc::Rc};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ForkId {
    First,
    Second,
}
impl ForkId {
    fn other(self) -> ForkId {
        match self {
            ForkId::First => ForkId::Second,
            ForkId::Second => ForkId::First,
        }
    }
}

pub trait Fork: Iterator + Sized {
    fn fork(self) -> (ForkIter<Self>, ForkIter<Self>) {
        let state = State {
            source: self,
            pending: VecDeque::new(),
            pending_fork_id: ForkId::First,
        };

        let state1 = Rc::new(RefCell::new(state));
        let state2 = Rc::clone(&state1);

        let it1 = ForkIter {
            fork_id: ForkId::First,
            state: state1,
        };
        let it2 = ForkIter {
            fork_id: ForkId::Second,
            state: state2,
        };

        (it1, it2)
    }
}

impl<I> Fork for I
where
    I: Iterator,
    I::Item: Clone,
{
}

#[derive(Clone)]
pub struct ForkIter<I>
where
    I: Iterator,
{
    fork_id: ForkId,
    state: Rc<RefCell<State<I>>>,
}
impl<I> Iterator for ForkIter<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.borrow_mut().next(self.fork_id)
    }
}
impl<I> FusedIterator for ForkIter<I>
where
    I: Iterator + FusedIterator,
    I::Item: Clone,
{
}
impl<I> Debug for ForkIter<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForkIter")
            .field("fork_id", &self.fork_id)
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Clone, Debug)]
struct State<I>
where
    I: Iterator,
{
    source: I,
    pending: VecDeque<I::Item>,
    pending_fork_id: ForkId, // The value doesn't matter when `pending` is empty
}
impl<I> State<I>
where
    I: Iterator,
    I::Item: Clone,
{
    fn next(&mut self, fork_id: ForkId) -> Option<I::Item> {
        let mut next = None;

        if self.pending_fork_id == fork_id {
            next = self.pending.pop_front();
        }

        while next.is_none() {
            match self.source.next() {
                Some(value) => {
                    debug_assert!(self.pending_fork_id != fork_id || self.pending.is_empty());
                    self.pending.push_back(value.clone());
                    self.pending_fork_id = fork_id.other();
                    next = Some(value);
                }
                None => break,
            }
        }

        next
    }
}

#[cfg(test)]
mod tests {

    use std::cell::Cell;

    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    enum Value {
        A { cloned: bool },
        B { cloned: bool },
    }
    impl Clone for Value {
        fn clone(&self) -> Self {
            match self {
                Self::A { .. } => Self::A { cloned: true },
                Self::B { .. } => Self::B { cloned: true },
            }
        }
    }

    #[test]
    fn it_works() {
        macro_rules! partitions {
            () => {{
                let consumed = Rc::new(Cell::new(0_usize));
                let (first, second) = [Value::A { cloned: false }, Value::B { cloned: false }]
                    .into_iter()
                    .inspect({
                        let consumed = Rc::clone(&consumed);
                        move |_| {
                            consumed.set(consumed.get() + 1);
                        }
                    })
                    .fork();

                (first, second, consumed)
            }};
        }

        // fffsss
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);

        // ffsfss
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);

        // ffssfs
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), None);

        // ffsssf
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), None);

        // fsffss
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);

        // fsfsfs
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), None);

        // fsfssf
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), None);

        // fssffs
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), None);

        // fssfsf
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), None);

        // fsssff
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(first.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);

        // sfffss
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);

        // sffsfs
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), None);

        // sffssf
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), None);

        // sfsffs
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), None);

        // sfsfsf
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), None);

        // sfssff
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);

        // ssfffs
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
        assert_eq!(second.next(), None);

        // ssffsf
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), None);

        // ssfsff
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);

        // sssfff
        let (mut first, mut second, consumed) = partitions!();
        assert_eq!(consumed.get(), 0);
        assert_eq!(second.next(), Some(Value::A { cloned: false }));
        assert_eq!(consumed.get(), 1);
        assert_eq!(second.next(), Some(Value::B { cloned: false }));
        assert_eq!(consumed.get(), 2);
        assert_eq!(second.next(), None);
        assert_eq!(first.next(), Some(Value::A { cloned: true }));
        assert_eq!(first.next(), Some(Value::B { cloned: true }));
        assert_eq!(first.next(), None);
    }
}
