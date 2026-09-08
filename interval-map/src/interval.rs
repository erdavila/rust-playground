mod endpoint;
mod interval_like;

use core::fmt::{Debug, Write};

pub use endpoint::*;
pub use interval_like::*;

use crate::{Relative, ValuesBetweenExist};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<K> {
    pub left: Endpoint<K>,
    pub right: Endpoint<K>,
}

impl<K> Interval<K> {
    pub fn new(left: Endpoint<K>, right: Endpoint<K>) -> Self {
        Interval { left, right }
    }

    pub fn contains(&self, value: &K) -> bool
    where
        K: Ord,
    {
        self.left_relative() <= value && self.right_relative() >= value
    }

    pub fn is_empty(&self) -> bool
    where
        K: ValuesBetweenExist,
    {
        if let (Endpoint::Open(l), Endpoint::Open(r)) = (self.left.as_ref(), self.right.as_ref()) {
            !K::values_between_exist(l, r)
        } else {
            self.left_relative() > self.right_relative()
        }
    }

    pub fn map<U>(self, mut f: impl FnMut(K) -> U) -> Interval<U> {
        self.map_endpoints(|e| e.map(&mut f))
    }

    fn map_endpoints<U>(self, mut f: impl FnMut(Endpoint<K>) -> Endpoint<U>) -> Interval<U> {
        Interval {
            left: f(self.left),
            right: f(self.right),
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

    pub fn left_relative(&self) -> Relative<&K> {
        match self.left.as_ref() {
            Endpoint::Open(x) => Relative::After(x),
            Endpoint::Closed(x) => Relative::At(x),
            Endpoint::Infinity => Relative::LeftInfinity,
        }
    }

    pub fn right_relative(&self) -> Relative<&K> {
        match self.right.as_ref() {
            Endpoint::Open(x) => Relative::Before(x),
            Endpoint::Closed(x) => Relative::At(x),
            Endpoint::Infinity => Relative::RightInfinity,
        }
    }
}

impl<K> Interval<&K> {
    pub fn cloned(self) -> Interval<K>
    where
        K: Clone,
    {
        self.map_endpoints(Endpoint::cloned)
    }

    pub fn copied(self) -> Interval<K>
    where
        K: Copy,
    {
        self.map_endpoints(Endpoint::copied)
    }
}

impl<K> IntervalLike<K> for Interval<K> {
    fn left(&self) -> Endpoint<&K> {
        self.left.as_ref()
    }

    fn right(&self) -> Endpoint<&K> {
        self.right.as_ref()
    }

    fn into_interval(self) -> Interval<K> {
        self
    }
}

impl<K: Debug> Debug for Interval<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.left() {
            Endpoint::Open(l) => {
                f.write_char('(')?;
                l.fmt(f)?;
            }
            Endpoint::Closed(l) => {
                f.write_char('[')?;
                l.fmt(f)?;
            }
            Endpoint::Infinity => f.write_str("(-∞")?,
        }

        f.write_str(", ")?;

        match self.right() {
            Endpoint::Open(r) => {
                r.fmt(f)?;
                f.write_char(')')?;
            }
            Endpoint::Closed(r) => {
                r.fmt(f)?;
                f.write_char(']')?;
            }
            Endpoint::Infinity => f.write_str("+∞)")?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::Endpoint::{Closed, Infinity, Open};

    #[test]
    fn contains() {
        const MIN: u32 = 5;
        const MAX: u32 = 10;

        let endpoints = (MIN..=MAX)
            .flat_map(|n| [Endpoint::Open(n), Endpoint::Closed(n)])
            .chain(core::iter::once(Endpoint::Infinity));

        for left in endpoints.clone() {
            for right in endpoints.clone() {
                let interval = Interval { left, right };

                let mut any_contained = false;
                for n in MIN - 1..=MAX + 1 {
                    let left_contained = match left {
                        Open(l) => n > l,
                        Closed(l) => n >= l,
                        Infinity => true,
                    };
                    let right_contained = match right {
                        Open(r) => n < r,
                        Closed(r) => n <= r,
                        Infinity => true,
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
        assert!(Interval::new(Open(7), Open(6)).is_empty());
        assert!(Interval::new(Open(7), Open(7)).is_empty());
        assert!(Interval::new(Open(7), Open(8)).is_empty());
        assert!(!Interval::new(Open(7), Open(9)).is_empty());
        // Open x Closed
        assert!(Interval::new(Open(7), Closed(6)).is_empty());
        assert!(Interval::new(Open(7), Closed(7)).is_empty());
        assert!(!Interval::new(Open(7), Closed(8)).is_empty());
        // Open x Infinity
        assert!(!Interval::new(Open(7), Infinity).is_empty());

        // Closed x Open
        assert!(Interval::new(Closed(7), Open(6)).is_empty());
        assert!(Interval::new(Closed(7), Open(7)).is_empty());
        assert!(!Interval::new(Closed(7), Open(8)).is_empty());
        // Closed x Closed
        assert!(Interval::new(Closed(7), Closed(6)).is_empty());
        assert!(!Interval::new(Closed(7), Closed(7)).is_empty());
        assert!(!Interval::new(Closed(7), Closed(8)).is_empty());
        // Closed x Infinity
        assert!(!Interval::new(Closed(7), Infinity).is_empty());

        // Infinity x Open
        assert!(!Interval::new(Infinity, Open(7)).is_empty());
        // Infinity x Closed
        assert!(!Interval::new(Infinity, Closed(7)).is_empty());
        // Infinity x Infinity
        assert!(!Interval::<i32>::new(Infinity, Infinity).is_empty());
    }
}
