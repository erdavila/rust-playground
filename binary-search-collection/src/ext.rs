//! Traits for extension methods.

use core::ops::{Bound, RangeBounds};

use crate::Range;
use crate::subslice::LocatedSubslice;

/// Extension methods for [`Range<usize>`].
pub trait RangeExt: Copy {
    fn from_start_and_len(start: usize, len: usize) -> Self;

    fn is_subrange_of(self, other: Self) -> bool;

    fn midpoint(self) -> usize;

    fn iter_from_midpoint(self) -> IterFromMidpoint;
}

impl RangeExt for Range {
    fn from_start_and_len(start: usize, len: usize) -> Self {
        Range {
            start,
            end: start + len,
        }
    }

    fn is_subrange_of(self, other: Self) -> bool {
        other.start <= self.start && self.end <= other.end
    }

    fn midpoint(self) -> usize {
        self.start.midpoint(self.end)
    }

    fn iter_from_midpoint(self) -> IterFromMidpoint {
        IterFromMidpoint {
            counter: 0,
            limit: self.end.saturating_sub(self.start),
            midpoint: self.start.midpoint(self.end),
        }
    }
}

pub struct IterFromMidpoint {
    counter: usize,
    limit: usize,
    midpoint: usize,
}

impl Iterator for IterFromMidpoint {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        (self.counter < self.limit).then(|| {
            let value = if self.counter.is_multiple_of(2) {
                self.midpoint + self.counter / 2
            } else {
                self.midpoint - self.counter / 2 - 1
            };

            self.counter += 1;
            value
        })
    }
}

/// Extension methods for [slice]s.
pub trait SliceExt<T>: AsRef<[T]> {
    fn range(&self) -> Range {
        let this = self.as_ref();
        (0..this.len()).into()
    }

    fn in_range<'a, O: Offset>(
        &'a self,
        range_bounds: impl RangeBounds<usize>,
        f: impl FnOnce(&'a [T]) -> O,
    ) -> O
    where
        T: 'a,
    {
        let start = match range_bounds.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => i + 1,
            Bound::Unbounded => 0,
        };

        let range_bounds = {
            let start_bound = Bound::Included(start);
            let end_bound = range_bounds.end_bound().cloned();
            (start_bound, end_bound)
        };

        let slice = &self.as_ref()[range_bounds];
        let output = f(slice);
        output.offset(start)
    }

    fn locate_first<P>(&self, predicate: P) -> Option<usize>
    where
        P: FnMut(&T) -> bool,
    {
        self.as_ref().iter().position(predicate)
    }

    fn locate_last<P>(&self, predicate: P) -> Option<usize>
    where
        P: FnMut(&T) -> bool,
    {
        self.as_ref().iter().rposition(predicate)
    }

    /// Search for an element in a slice from the midpoint towards both ends, returning its index.
    ///
    /// The returned index is for the element that is closer to the midpoint.
    fn locate_from_midpoint<P>(&self, mut predicate: P) -> Option<usize>
    where
        P: FnMut(&T) -> bool,
    {
        let this = self.as_ref();
        this.range()
            .iter_from_midpoint()
            .find(|i| predicate(&this[*i]))
    }

    fn extend_subslice_range_to_delimiters<P>(
        &self,
        subslice_range: Range,
        mut is_delimiter: P,
    ) -> LocatedSubslice
    where
        P: FnMut(&T) -> bool,
    {
        let this = self.as_ref();

        let (subslice_start, consumed_start) = self
            .in_range(..subslice_range.start, |slice| {
                slice.locate_last(&mut is_delimiter)
            })
            .map_or((0, 0), |i| (i + 1, i));

        let (subslice_end, consumed_end) = self
            .in_range(subslice_range.end.., |slice| {
                slice.locate_first(&mut is_delimiter)
            })
            .map_or((this.len(), this.len()), |i| (i, i + 1));

        LocatedSubslice {
            subslice_range: (subslice_start..subslice_end).into(),
            consumed_range: (consumed_start..consumed_end).into(),
        }
    }

    fn subslice_range_from_midpoint_to_delimiters<P>(&self, is_delimiter: P) -> LocatedSubslice
    where
        P: FnMut(&T) -> bool,
    {
        let mid = self.range().midpoint();
        let midpoint_range = Range::from_start_and_len(mid, 0);
        self.extend_subslice_range_to_delimiters(midpoint_range, is_delimiter)
    }
}

impl<T> SliceExt<T> for [T] {}

pub trait Offset {
    #[must_use]
    fn offset(self, amount: usize) -> Self;
}

impl Offset for usize {
    fn offset(self, amount: usize) -> Self {
        self + amount
    }
}

impl<O: Offset> Offset for Option<O> {
    fn offset(self, amount: usize) -> Self {
        self.map(|x| x.offset(amount))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    mod range_ext {
        use alloc::vec::Vec;

        use super::*;

        #[test]
        #[expect(clippy::reversed_empty_ranges)]
        fn iter_from_midpoint() {
            fn from_midpoint_as_vec(ops_range: core::ops::Range<usize>) -> Vec<usize> {
                Range::from(ops_range).iter_from_midpoint().collect()
            }

            macro_rules! assert_or {
                ($vec:expr, $pattern1:pat_param | $pattern2:pat_param) => {
                    assert!(matches!($vec.as_slice(), $pattern1 | $pattern2));
                };
            }

            assert_eq!(from_midpoint_as_vec(5..4), []);
            assert_eq!(from_midpoint_as_vec(5..5), []);
            assert_eq!(from_midpoint_as_vec(5..6), [5]);
            assert_eq!(from_midpoint_as_vec(5..7), [6, 5]);
            assert_or!(from_midpoint_as_vec(5..8), [6, 5, 7] | [6, 7, 5]);
            assert_eq!(from_midpoint_as_vec(5..9), [7, 6, 8, 5]);
            assert_or!(
                from_midpoint_as_vec(5..10),
                [7, 6, 8, 5, 9] | [7, 8, 6, 9, 5]
            );
            assert_eq!(from_midpoint_as_vec(5..11), [8, 7, 9, 6, 10, 5]);
            assert_or!(
                from_midpoint_as_vec(5..12),
                [8, 7, 9, 6, 10, 5, 11] | [8, 9, 7, 10, 6, 11, 5]
            );
        }
    }
}
