use std::{cell::RefCell, collections::VecDeque, fmt::Debug, iter::FusedIterator, rc::Rc};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PartitionId {
    True,
    False,
}
impl PartitionId {
    fn other(self) -> PartitionId {
        match self {
            PartitionId::True => PartitionId::False,
            PartitionId::False => PartitionId::True,
        }
    }
}
impl From<bool> for PartitionId {
    fn from(value: bool) -> Self {
        if value {
            PartitionId::True
        } else {
            PartitionId::False
        }
    }
}

pub trait PartitionIters: Iterator + Sized {
    fn partition_iters<P>(self, predicate: P) -> (PartitionIter<Self, P>, PartitionIter<Self, P>)
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let state = State {
            source: self,
            predicate,
            pending: VecDeque::new(),
            pending_partition_id: PartitionId::False,
        };

        let state1 = Rc::new(RefCell::new(state));
        let state2 = Rc::clone(&state1);

        let it1 = PartitionIter {
            partition_id: PartitionId::True,
            state: state1,
        };
        let it2 = PartitionIter {
            partition_id: PartitionId::False,
            state: state2,
        };

        (it1, it2)
    }
}

impl<I> PartitionIters for I where I: Iterator {}

#[derive(Clone)]
pub struct PartitionIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    partition_id: PartitionId,
    state: Rc<RefCell<State<I, P>>>,
}
impl<I, P> Iterator for PartitionIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.borrow_mut().next(self.partition_id)
    }
}
impl<I, P> FusedIterator for PartitionIter<I, P>
where
    I: Iterator + FusedIterator,
    P: FnMut(&I::Item) -> bool,
{
}
impl<I, P> Debug for PartitionIter<I, P>
where
    I: Iterator + Debug,
    I::Item: Debug,
    P: FnMut(&I::Item) -> bool + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartitionIter")
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Clone, Debug)]
struct State<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    source: I,
    predicate: P,
    pending: VecDeque<I::Item>,
    pending_partition_id: PartitionId, // The value doesn't matter when `pending` is empty
}
impl<I, P> State<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn next(&mut self, partition_id: PartitionId) -> Option<I::Item> {
        let mut next = None;

        if self.pending_partition_id == partition_id {
            next = self.pending.pop_front();
        }

        while next.is_none() {
            match self.source.next() {
                Some(value) => {
                    if PartitionId::from((self.predicate)(&value)) == partition_id {
                        next = Some(value)
                    } else {
                        debug_assert!(
                            self.pending_partition_id != partition_id || self.pending.is_empty()
                        );
                        self.pending.push_back(value);
                        self.pending_partition_id = partition_id.other();
                    }
                }
                None => break,
            }
        }

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inputs = [
            ['A', 'B', 'x', 'y'],
            ['A', 'x', 'B', 'y'],
            ['A', 'x', 'y', 'B'],
            ['x', 'A', 'B', 'y'],
            ['x', 'A', 'y', 'B'],
            ['x', 'y', 'A', 'B'],
        ];

        for input in inputs {
            macro_rules! partitions {
                () => {
                    input.into_iter().partition_iters(|c| c.is_uppercase())
                };
            }

            // UUUddd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);

            // UUdUdd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);

            // UUddUd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), None);

            // UUdddU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), None);

            // UdUUdd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);

            // UdUdUd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), None);

            // UdUddU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), None);

            // UddUUd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), None);

            // UddUdU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), None);

            // UdddUU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);

            // dUUUdd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);

            // dUUdUd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), None);

            // dUUddU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), None);

            // dUdUUd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), None);

            // dUdUdU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), None);

            // dUddUU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);

            // ddUUUd
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
            assert_eq!(downcase.next(), None);

            // ddUUdU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), None);

            // ddUdUU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);

            // dddUUU
            let (mut uppercase, mut downcase) = partitions!();
            assert_eq!(downcase.next(), Some('x'));
            assert_eq!(downcase.next(), Some('y'));
            assert_eq!(downcase.next(), None);
            assert_eq!(uppercase.next(), Some('A'));
            assert_eq!(uppercase.next(), Some('B'));
            assert_eq!(uppercase.next(), None);
        }
    }
}
