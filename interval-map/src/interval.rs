mod endpoint;
mod interval_like;

use core::cmp::Ordering;
use core::fmt::Debug;

pub use endpoint::*;
pub use interval_like::*;

use crate::ValuesBetweenExist;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<K> {
    pub left: LeftEndpoint<K>,
    pub right: RightEndpoint<K>,
}

impl<K> Interval<K> {
    pub fn new(left: LeftEndpoint<K>, right: RightEndpoint<K>) -> Self {
        Interval { left, right }
    }

    pub fn map<U>(self, mut f: impl FnMut(K) -> U) -> Interval<U> {
        Interval {
            left: self.left.map(&mut f),
            right: self.right.map(&mut f),
        }
    }

    pub fn as_ref(&self) -> Interval<&K> {
        Interval {
            left: self.left.as_ref(),
            right: self.right.as_ref(),
        }
    }

    pub fn as_mut(&mut self) -> Interval<&mut K> {
        Interval {
            left: self.left.as_mut(),
            right: self.right.as_mut(),
        }
    }
}

impl<K> Interval<K>
where
    K: Ord,
    // TODO: remove these bounds.
    K: Debug,
{
    pub fn contains(&self, value: &K) -> bool {
        self.left <= *value && self.right >= value
    }

    pub fn is_subinterval_of(&self, other: &Interval<K>) -> bool
    where
        // TODO: remove bounds
        K: Debug,
    {
        other.left <= self.left && self.right <= other.right
    }

    #[expect(clippy::unnested_or_patterns)]
    pub fn is_proper_subinterval_of(&self, other: &Interval<K>) -> bool {
        matches!(
            (other.left.cmp(&self.left), self.right.cmp(&other.right)),
            (Ordering::Less, Ordering::Less)
                | (Ordering::Less, Ordering::Equal)
                | (Ordering::Equal, Ordering::Less)
        )
    }
}

impl<K> Interval<K>
where
    K: ValuesBetweenExist,
{
    pub fn is_empty(&self) -> bool {
        if let (LeftEndpoint::Open(l), RightEndpoint::Open(r)) =
            (self.left.as_ref(), self.right.as_ref())
        {
            !K::values_between_exist(l, r)
        } else {
            self.left > self.right
        }
    }
}

impl<K> Interval<&K> {
    #[must_use]
    pub fn cloned(self) -> Interval<K>
    where
        K: Clone,
    {
        self.map(Clone::clone)
    }

    #[must_use]
    pub fn copied(self) -> Interval<K>
    where
        K: Copy,
    {
        self.map(|x| *x)
    }
}

impl<K> IntervalLike<K> for Interval<K> {
    fn left(&self) -> LeftEndpoint<&K> {
        self.left.as_ref()
    }

    fn right(&self) -> RightEndpoint<&K> {
        self.right.as_ref()
    }

    fn into_interval(self) -> Interval<K> {
        self
    }
}

impl<K: Debug> Debug for Interval<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.left.fmt(f)?;
        f.write_str(", ")?;
        self.right.fmt(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::iter::once;

    use super::*;
    use crate::interval::LeftEndpoint::{Closed as LClosed, Infinity as LInfinity, Open as LOpen};
    use crate::interval::RightEndpoint::{Closed as RClosed, Infinity as RInfinity, Open as ROpen};

    #[test]
    fn contains() {
        const MIN: u32 = 5;
        const MAX: u32 = 10;

        let lefts = (MIN..=MAX)
            .flat_map(|n| [LOpen(n), LClosed(n)])
            .chain(once(LInfinity));

        let rights = (MIN..=MAX)
            .flat_map(|n| [ROpen(n), RClosed(n)])
            .chain(once(RInfinity));

        for left in lefts {
            for right in rights.clone() {
                let interval = Interval { left, right };

                let mut any_contained = false;
                for n in MIN - 1..=MAX + 1 {
                    let left_contained = match left {
                        LOpen(l) => n > l,
                        LClosed(l) => n >= l,
                        LInfinity => true,
                    };
                    let right_contained = match right {
                        ROpen(r) => n < r,
                        RClosed(r) => n <= r,
                        RInfinity => true,
                    };
                    let contained = left_contained && right_contained;

                    assert_eq!(interval.contains(&n), contained, "{interval:?} {n}");

                    any_contained |= contained;
                }

                assert_eq!(interval.is_empty(), !any_contained, "{interval:?}");
            }
        }
    }

    #[test]
    fn is_empty() {
        // Open x Open
        assert!(Interval::new(LOpen(7), ROpen(6)).is_empty());
        assert!(Interval::new(LOpen(7), ROpen(7)).is_empty());
        assert!(Interval::new(LOpen(7), ROpen(8)).is_empty());
        assert!(!Interval::new(LOpen(7), ROpen(9)).is_empty());
        // Open x Closed
        assert!(Interval::new(LOpen(7), RClosed(6)).is_empty());
        assert!(Interval::new(LOpen(7), RClosed(7)).is_empty());
        assert!(!Interval::new(LOpen(7), RClosed(8)).is_empty());
        // Open x Infinity
        assert!(!Interval::new(LOpen(7), RInfinity).is_empty());

        // Closed x Open
        assert!(Interval::new(LClosed(7), ROpen(6)).is_empty());
        assert!(Interval::new(LClosed(7), ROpen(7)).is_empty());
        assert!(!Interval::new(LClosed(7), ROpen(8)).is_empty());
        // Closed x Closed
        assert!(Interval::new(LClosed(7), RClosed(6)).is_empty());
        assert!(!Interval::new(LClosed(7), RClosed(7)).is_empty());
        assert!(!Interval::new(LClosed(7), RClosed(8)).is_empty());
        // Closed x Infinity
        assert!(!Interval::new(LClosed(7), RInfinity).is_empty());

        // Infinity x Open
        assert!(!Interval::new(LInfinity, ROpen(7)).is_empty());
        // Infinity x Closed
        assert!(!Interval::new(LInfinity, RClosed(7)).is_empty());
        // Infinity x Infinity
        assert!(!Interval::<i32>::new(LInfinity, RInfinity).is_empty());
    }
}
