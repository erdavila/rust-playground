use core::cmp::Ordering;
use core::fmt::{Debug, Write as _};
use core::ops::Bound;

macro_rules! impl_partial_for_lhs_ref {
    ($lhs:ty, $rhs:ty) => {
        impl<K: Ord> PartialEq<$rhs> for &$lhs {
            fn eq(&self, other: &$rhs) -> bool {
                (*self).eq(other)
            }
        }
        impl<K: Ord> PartialOrd<$rhs> for &$lhs {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                (*self).partial_cmp(other)
            }
        }
    };
}

macro_rules! impl_partial_for_rhs_ref {
    ($lhs:ty, $rhs:ty) => {
        impl<K: Ord> PartialEq<&$rhs> for $lhs {
            fn eq(&self, other: &&$rhs) -> bool {
                self.eq(*other)
            }
        }
        impl<K: Ord> PartialOrd<&$rhs> for $lhs {
            fn partial_cmp(&self, other: &&$rhs) -> Option<Ordering> {
                self.partial_cmp(*other)
            }
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeftEndpoint<K> {
    Open(K),
    Closed(K),
    Infinity,
}

impl<K> LeftEndpoint<K> {
    pub fn map<U>(self, mut f: impl FnMut(K) -> U) -> LeftEndpoint<U> {
        match self {
            LeftEndpoint::Open(k) => LeftEndpoint::Open(f(k)),
            LeftEndpoint::Closed(k) => LeftEndpoint::Closed(f(k)),
            LeftEndpoint::Infinity => LeftEndpoint::Infinity,
        }
    }

    pub fn as_ref(&self) -> LeftEndpoint<&K> {
        match self {
            LeftEndpoint::Open(k) => LeftEndpoint::Open(k),
            LeftEndpoint::Closed(k) => LeftEndpoint::Closed(k),
            LeftEndpoint::Infinity => LeftEndpoint::Infinity,
        }
    }

    pub fn as_mut(&mut self) -> LeftEndpoint<&mut K> {
        match self {
            LeftEndpoint::Open(k) => LeftEndpoint::Open(k),
            LeftEndpoint::Closed(k) => LeftEndpoint::Closed(k),
            LeftEndpoint::Infinity => LeftEndpoint::Infinity,
        }
    }

    pub fn into_range_bound(self) -> Bound<K> {
        match self {
            LeftEndpoint::Open(k) => Bound::Excluded(k),
            LeftEndpoint::Closed(k) => Bound::Included(k),
            LeftEndpoint::Infinity => Bound::Unbounded,
        }
    }

    pub fn opposite(&self) -> Option<RightEndpoint<&K>> {
        match self {
            LeftEndpoint::Open(k) => Some(RightEndpoint::Closed(k)),
            LeftEndpoint::Closed(k) => Some(RightEndpoint::Open(k)),
            LeftEndpoint::Infinity => None,
        }
    }

    fn cmp_endpoint(&self) -> CmpEndpoint<&K> {
        match self {
            LeftEndpoint::Open(x) => CmpEndpoint::After(x),
            LeftEndpoint::Closed(x) => CmpEndpoint::At(x),
            LeftEndpoint::Infinity => CmpEndpoint::LeftInfinity,
        }
    }
}

impl<K> LeftEndpoint<&K> {
    #[must_use]
    pub fn cloned(self) -> LeftEndpoint<K>
    where
        K: Clone,
    {
        self.map(K::clone)
    }

    #[must_use]
    pub fn copied(self) -> LeftEndpoint<K>
    where
        K: Copy,
    {
        self.map(|k| *k)
    }
}

// LeftEndpoint<K> <?> LeftEndpoint<K>
impl<K: Ord> PartialOrd for LeftEndpoint<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K: Ord> Ord for LeftEndpoint<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_endpoint().cmp(&other.cmp_endpoint())
    }
}

// &LeftEndpoint<K> <?> LeftEndpoint<K>
impl_partial_for_lhs_ref!(LeftEndpoint<K>, LeftEndpoint<K>);

// LeftEndpoint<K> <?> &LeftEndpoint<K>
impl_partial_for_rhs_ref!(LeftEndpoint<K>, LeftEndpoint<K>);

// LeftEndpoint<K> <?> K
impl<K: Ord> PartialEq<K> for LeftEndpoint<K> {
    fn eq(&self, other: &K) -> bool {
        self.cmp_endpoint().eq(&CmpEndpoint::At(other))
    }
}
impl<K: Ord> PartialOrd<K> for LeftEndpoint<K> {
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        self.cmp_endpoint().partial_cmp(&CmpEndpoint::At(other))
    }
}

// LeftEndpoint<K> <?> &K
impl_partial_for_rhs_ref!(LeftEndpoint<K>, K);

// LeftEndpoint<K> <?> RightEndpoint<K>
impl<K: Ord> PartialEq<RightEndpoint<K>> for LeftEndpoint<K> {
    fn eq(&self, other: &RightEndpoint<K>) -> bool {
        self.cmp_endpoint().eq(&other.cmp_endpoint())
    }
}
impl<K: Ord> PartialOrd<RightEndpoint<K>> for LeftEndpoint<K> {
    fn partial_cmp(&self, other: &RightEndpoint<K>) -> Option<Ordering> {
        self.cmp_endpoint().partial_cmp(&other.cmp_endpoint())
    }
}

// &LeftEndpoint<K> <?> RightEndpoint<K>
impl_partial_for_lhs_ref!(LeftEndpoint<K>, RightEndpoint<K>);

// LeftEndpoint<K> <?> &RightEndpoint<K>
impl_partial_for_rhs_ref!(LeftEndpoint<K>, RightEndpoint<K>);

impl<K> From<Bound<K>> for LeftEndpoint<K> {
    fn from(value: Bound<K>) -> Self {
        match value {
            Bound::Included(k) => LeftEndpoint::Closed(k),
            Bound::Excluded(k) => LeftEndpoint::Open(k),
            Bound::Unbounded => LeftEndpoint::Infinity,
        }
    }
}

impl<K> From<LeftEndpoint<K>> for Bound<K> {
    fn from(value: LeftEndpoint<K>) -> Self {
        value.into_range_bound()
    }
}

impl<K: Debug> Debug for LeftEndpoint<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Open(l) => {
                f.write_char('(')?;
                l.fmt(f)?;
            }
            Self::Closed(l) => {
                f.write_char('[')?;
                l.fmt(f)?;
            }
            Self::Infinity => {
                f.write_str("(-")?;
                if f.alternate() {
                    f.write_char('∞')?;
                } else {
                    f.write_str("inf")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RightEndpoint<K> {
    Open(K),
    Closed(K),
    Infinity,
}

impl<K> RightEndpoint<K> {
    pub fn map<U>(self, mut f: impl FnMut(K) -> U) -> RightEndpoint<U> {
        match self {
            RightEndpoint::Open(k) => RightEndpoint::Open(f(k)),
            RightEndpoint::Closed(k) => RightEndpoint::Closed(f(k)),
            RightEndpoint::Infinity => RightEndpoint::Infinity,
        }
    }

    pub fn as_ref(&self) -> RightEndpoint<&K> {
        match self {
            RightEndpoint::Open(k) => RightEndpoint::Open(k),
            RightEndpoint::Closed(k) => RightEndpoint::Closed(k),
            RightEndpoint::Infinity => RightEndpoint::Infinity,
        }
    }

    pub fn as_mut(&mut self) -> RightEndpoint<&mut K> {
        match self {
            RightEndpoint::Open(k) => RightEndpoint::Open(k),
            RightEndpoint::Closed(k) => RightEndpoint::Closed(k),
            RightEndpoint::Infinity => RightEndpoint::Infinity,
        }
    }

    pub fn into_range_bound(self) -> Bound<K> {
        match self {
            RightEndpoint::Open(k) => Bound::Excluded(k),
            RightEndpoint::Closed(k) => Bound::Included(k),
            RightEndpoint::Infinity => Bound::Unbounded,
        }
    }

    pub fn opposite(&self) -> Option<LeftEndpoint<&K>> {
        match self {
            RightEndpoint::Open(k) => Some(LeftEndpoint::Closed(k)),
            RightEndpoint::Closed(k) => Some(LeftEndpoint::Open(k)),
            RightEndpoint::Infinity => None,
        }
    }

    fn cmp_endpoint(&self) -> CmpEndpoint<&K> {
        match self {
            RightEndpoint::Open(x) => CmpEndpoint::Before(x),
            RightEndpoint::Closed(x) => CmpEndpoint::At(x),
            RightEndpoint::Infinity => CmpEndpoint::RightInfinity,
        }
    }
}

impl<K> RightEndpoint<&K> {
    #[must_use]
    pub fn cloned(self) -> RightEndpoint<K>
    where
        K: Clone,
    {
        self.map(K::clone)
    }

    #[must_use]
    pub fn copied(self) -> RightEndpoint<K>
    where
        K: Copy,
    {
        self.map(|k| *k)
    }
}

// RightEndpoint<K> <?> RightEndpoint<K>
impl<K: Ord> PartialOrd for RightEndpoint<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K: Ord> Ord for RightEndpoint<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_endpoint().cmp(&other.cmp_endpoint())
    }
}

// &RightEndpoint<K> <?> RightEndpoint<K>
impl_partial_for_lhs_ref!(RightEndpoint<K>, RightEndpoint<K>);

// RightEndpoint<K> <?> &RightEndpoint<K>
impl_partial_for_rhs_ref!(RightEndpoint<K>, RightEndpoint<K>);

// RightEndpoint<K> <?> K
impl<K: Ord> PartialEq<K> for RightEndpoint<K> {
    fn eq(&self, other: &K) -> bool {
        self.cmp_endpoint().eq(&CmpEndpoint::At(other))
    }
}
impl<K: Ord> PartialOrd<K> for RightEndpoint<K> {
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        self.cmp_endpoint().partial_cmp(&CmpEndpoint::At(other))
    }
}

// RightEndpoint<K> <?> &K
impl_partial_for_rhs_ref!(RightEndpoint<K>, K);

// RightEndpoint<K> <?> LeftEndpoint<K>
impl<K: Ord> PartialEq<LeftEndpoint<K>> for RightEndpoint<K> {
    fn eq(&self, other: &LeftEndpoint<K>) -> bool {
        self.cmp_endpoint().eq(&other.cmp_endpoint())
    }
}
impl<K: Ord> PartialOrd<LeftEndpoint<K>> for RightEndpoint<K> {
    fn partial_cmp(&self, other: &LeftEndpoint<K>) -> Option<Ordering> {
        self.cmp_endpoint().partial_cmp(&other.cmp_endpoint())
    }
}

// &RightEndpoint<K> <?> LeftEndpoint<K>
impl_partial_for_lhs_ref!(RightEndpoint<K>, LeftEndpoint<K>);

// RightEndpoint<K> <?> &LeftEndpoint<K>
impl_partial_for_rhs_ref!(RightEndpoint<K>, LeftEndpoint<K>);

impl<K> From<Bound<K>> for RightEndpoint<K> {
    fn from(value: Bound<K>) -> Self {
        match value {
            Bound::Included(k) => RightEndpoint::Closed(k),
            Bound::Excluded(k) => RightEndpoint::Open(k),
            Bound::Unbounded => RightEndpoint::Infinity,
        }
    }
}

impl<K> From<RightEndpoint<K>> for Bound<K> {
    fn from(value: RightEndpoint<K>) -> Self {
        value.into_range_bound()
    }
}

impl<K: Debug> Debug for RightEndpoint<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Open(r) => {
                r.fmt(f)?;
                f.write_char(')')?;
            }
            Self::Closed(r) => {
                r.fmt(f)?;
                f.write_char(']')?;
            }
            Self::Infinity => {
                f.write_char('+')?;
                if f.alternate() {
                    f.write_char('∞')?;
                } else {
                    f.write_str("inf")?;
                }
                f.write_char(')')?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CmpEndpoint<K> {
    LeftInfinity,
    Before(K),
    At(K),
    After(K),
    RightInfinity,
}
impl<K: Ord> PartialOrd for CmpEndpoint<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K: Ord> Ord for CmpEndpoint<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        #[expect(clippy::unnested_or_patterns)]
        match (self, other) {
            (CmpEndpoint::Before(this), CmpEndpoint::Before(other))
            | (CmpEndpoint::At(this), CmpEndpoint::At(other))
            | (CmpEndpoint::After(this), CmpEndpoint::After(other)) => this.cmp(other),

            (CmpEndpoint::Before(this), CmpEndpoint::At(other))
            | (CmpEndpoint::Before(this), CmpEndpoint::After(other))
            | (CmpEndpoint::At(this), CmpEndpoint::After(other)) => match this.cmp(other) {
                Ordering::Equal => Ordering::Less,
                ord => ord,
            },

            (CmpEndpoint::At(this), CmpEndpoint::Before(other))
            | (CmpEndpoint::After(this), CmpEndpoint::Before(other))
            | (CmpEndpoint::After(this), CmpEndpoint::At(other)) => match this.cmp(other) {
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            },

            (CmpEndpoint::LeftInfinity, CmpEndpoint::LeftInfinity)
            | (CmpEndpoint::RightInfinity, CmpEndpoint::RightInfinity) => Ordering::Equal,

            (CmpEndpoint::LeftInfinity, _) | (_, CmpEndpoint::RightInfinity) => Ordering::Less,

            (_, CmpEndpoint::LeftInfinity) | (CmpEndpoint::RightInfinity, _) => Ordering::Greater,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::iter::once;

    use super::*;

    mod left_endpoint {
        use super::*;

        #[test]
        fn cmp() {
            // Values sorted as expected.
            let values = once(LeftEndpoint::Infinity).chain(
                [7, 10]
                    .into_iter()
                    .flat_map(|n| [LeftEndpoint::Closed(n), LeftEndpoint::Open(n)]),
            );

            for (i1, l1) in values.clone().enumerate() {
                for (i2, l2) in values.clone().enumerate() {
                    let expected = i1.cmp(&i2);

                    assert_eq!(l1.cmp(&l2), expected);
                }
            }
        }

        #[test]
        fn partial_cmp_value() {
            assert_eq!(
                LeftEndpoint::Infinity.partial_cmp(&i32::MIN),
                Some(Ordering::Less)
            );
            assert_eq!(LeftEndpoint::Infinity.partial_cmp(&7), Some(Ordering::Less));
            assert_eq!(
                LeftEndpoint::Closed(6).partial_cmp(&7),
                Some(Ordering::Less)
            );
            assert_eq!(LeftEndpoint::Open(6).partial_cmp(&7), Some(Ordering::Less));
            assert_eq!(
                LeftEndpoint::Closed(7).partial_cmp(&7),
                Some(Ordering::Equal)
            );
            assert_eq!(
                LeftEndpoint::Open(7).partial_cmp(&7),
                Some(Ordering::Greater)
            );
            assert_eq!(
                LeftEndpoint::Closed(8).partial_cmp(&7),
                Some(Ordering::Greater)
            );
            assert_eq!(
                LeftEndpoint::Open(8).partial_cmp(&7),
                Some(Ordering::Greater)
            );
        }
    }

    mod right_endpoint {
        use super::*;

        #[test]
        fn cmp() {
            // Values sorted as expected.
            let values = [7, 10]
                .into_iter()
                .flat_map(|n| [RightEndpoint::Open(n), RightEndpoint::Closed(n)])
                .chain(once(RightEndpoint::Infinity));

            for (i1, r1) in values.clone().enumerate() {
                for (i2, r2) in values.clone().enumerate() {
                    let expected = i1.cmp(&i2);

                    assert_eq!(r1.cmp(&r2), expected);
                }
            }
        }

        #[test]
        fn partial_cmp_value() {
            assert_eq!(RightEndpoint::Open(6).partial_cmp(&7), Some(Ordering::Less));
            assert_eq!(
                RightEndpoint::Closed(6).partial_cmp(&7),
                Some(Ordering::Less)
            );
            assert_eq!(RightEndpoint::Open(7).partial_cmp(&7), Some(Ordering::Less));
            assert_eq!(
                RightEndpoint::Closed(7).partial_cmp(&7),
                Some(Ordering::Equal)
            );
            assert_eq!(
                RightEndpoint::Open(8).partial_cmp(&7),
                Some(Ordering::Greater)
            );
            assert_eq!(
                RightEndpoint::Closed(8).partial_cmp(&7),
                Some(Ordering::Greater)
            );
            assert_eq!(
                RightEndpoint::Infinity.partial_cmp(&7),
                Some(Ordering::Greater)
            );
            assert_eq!(
                RightEndpoint::Infinity.partial_cmp(&i32::MAX),
                Some(Ordering::Greater)
            );
        }
    }

    #[test]
    fn left_endpoint_partial_cmp_right_endpoint() {
        let lefts = [
            (0, LeftEndpoint::Infinity),
            (35, LeftEndpoint::Closed(3)),
            (36, LeftEndpoint::Open(3)),
            (75, LeftEndpoint::Closed(7)),
            (76, LeftEndpoint::Open(7)),
        ];
        let rights = [
            (34, RightEndpoint::Open(3)),
            (35, RightEndpoint::Closed(3)),
            (74, RightEndpoint::Open(7)),
            (75, RightEndpoint::Closed(7)),
            (100, RightEndpoint::Infinity),
        ];

        for (ln, l) in lefts {
            for (rn, r) in rights {
                let expected = ln.cmp(&rn);

                assert_eq!(l.partial_cmp(&r), Some(expected));
                assert_eq!(r.partial_cmp(&l), Some(expected.reverse()));
            }
        }
    }
}
