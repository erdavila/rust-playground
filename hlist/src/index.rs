//! Provides the [`Index`] trait and related types.
//!
//! See the [`HCons`] methods [`get_index_by_type`], [`get_by_index`] and
//! [`get_by_index_mut`] for examples of how to use this trait.
//!
//! [`HCons`]: crate::HCons
//! [`get_index_by_type`]: crate::HCons::get_index_by_type
//! [`get_by_index`]: crate::HCons::get_by_index
//! [`get_by_index_mut`]: crate::HCons::get_by_index_mut

use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;

/// A type that represents the index of a type or value in an [`HList`](crate::HList).
pub trait Index: Copy + Debug + Default + Hash + Ord {
    /// The numeric value of the index.
    const VALUE: usize;

    /// Returns the numeric value of the index.
    fn value(&self) -> usize {
        Self::VALUE
    }
}

/// The index of the first type in a list of types.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord)]
pub struct Zero;
impl Index for Zero {
    const VALUE: usize = 0;
}
impl<I: Index> PartialEq<I> for Zero {
    fn eq(&self, other: &I) -> bool {
        self.value().eq(&other.value())
    }
}
impl<I: Index> PartialOrd<I> for Zero {
    fn partial_cmp(&self, other: &I) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

/// The index of the type after a given index.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord)]
pub struct Succ<I: Index>(pub I);
impl<I: Index> Index for Succ<I> {
    const VALUE: usize = I::VALUE + 1;
}
impl<I: Index, OtherI: Index> PartialEq<OtherI> for Succ<I> {
    fn eq(&self, other: &OtherI) -> bool {
        self.value().eq(&other.value())
    }
}
impl<I: Index, OtherI: Index> PartialOrd<OtherI> for Succ<I> {
    fn partial_cmp(&self, other: &OtherI) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let i0 = Zero;
        let i1 = Succ(i0);
        let i2 = Succ(i1);

        assert!(i0.eq(&i0));
        assert!(!i0.eq(&i1));
        assert!(!i0.eq(&i2));

        assert!(!i1.eq(&i0));
        assert!(i1.eq(&i1));
        assert!(!i1.eq(&i2));

        assert!(!i2.eq(&i0));
        assert!(!i2.eq(&i1));
        assert!(i2.eq(&i2));
    }

    #[test]
    fn partial_ord() {
        let i0 = Zero;
        let i1 = Succ(i0);
        let i2 = Succ(i1);

        assert_eq!(i0.partial_cmp(&i0), Some(Ordering::Equal));
        assert_eq!(i0.partial_cmp(&i1), Some(Ordering::Less));
        assert_eq!(i0.partial_cmp(&i2), Some(Ordering::Less));

        assert_eq!(i1.partial_cmp(&i0), Some(Ordering::Greater));
        assert_eq!(i1.partial_cmp(&i1), Some(Ordering::Equal));
        assert_eq!(i1.partial_cmp(&i2), Some(Ordering::Less));

        assert_eq!(i2.partial_cmp(&i0), Some(Ordering::Greater));
        assert_eq!(i2.partial_cmp(&i1), Some(Ordering::Greater));
        assert_eq!(i2.partial_cmp(&i2), Some(Ordering::Equal));
    }
}
