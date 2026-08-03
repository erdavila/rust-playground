//! Traits for extension methods.

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::ops::{Bound, RangeBounds};

use crate::line::{self, CR, LF, LineBytes, LineEnd};
use crate::subslice::{self, LocatedSubslice};
use crate::{Comparison, LocatedItem, Range, SearchResult};

/// Extension methods for [`Range<usize>`].
pub trait RangeExt: Copy {
    fn from_start_and_len(start: usize, len: usize) -> Self;

    fn is_subrange_of(self, other: Self) -> bool;

    fn len(self) -> usize;
    fn is_empty(self) -> bool;
    fn midpoint(self) -> usize;

    /// Returns an iterator that generates all the values in the range from the midpoint towards both ends.
    ///
    /// Check the [Values with Gaps](crate#values-with-gaps) example.
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

    fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    fn is_empty(self) -> bool {
        self.len() == 0
    }

    fn midpoint(self) -> usize {
        self.start.midpoint(self.end)
    }

    fn iter_from_midpoint(self) -> IterFromMidpoint {
        IterFromMidpoint {
            counter: 0,
            limit: self.len(),
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
    /// The function [`subslice::binary_search`] as an extension method.
    ///
    /// ```
    /// # use binary_search_collection::{Range, SearchResult};
    /// # use binary_search_collection::subslice::{self, LocatedSubslice};
    /// # use binary_search_collection::ext::SliceExt as _;
    /// # fn f<T: Ord, E>(
    /// #     slice: &[T],
    /// #     subslice: &[T],
    /// #     mut locate_subslice_in: impl FnMut(&[T]) -> Result<Option<LocatedSubslice>, E> + Copy,
    /// # ) -> SearchResult<Range, E> {
    /// #     if unimplemented!() {
    /// slice.subslice_binary_search(subslice, locate_subslice_in)
    /// #     } else {
    /// // is equivalent to:
    /// subslice::binary_search(subslice, slice, locate_subslice_in)
    /// #     }
    /// # }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    fn subslice_binary_search<'a, E>(
        &'a self,
        subslice: &[T],
        locate_subslice_in: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
    ) -> SearchResult<Range, E>
    where
        T: Ord + 'a,
    {
        subslice::binary_search(subslice, self.as_ref(), locate_subslice_in)
    }

    /// The function [`subslice::binary_search_by_key`] as an extension method.
    ///
    /// ```
    /// # use binary_search_collection::{subslice, LocatedItem, Range, SearchResult};
    /// # use binary_search_collection::ext::SliceExt as _;
    /// # use std::borrow::Borrow;
    /// # fn f<T, V, Q, E>(
    /// #     slice: &[T],
    /// #     target_value: &Q,
    /// #     mut locate_item_in: impl FnMut(&[T]) -> Result<Option<LocatedItem<V>>, E> + Copy,
    /// # ) -> SearchResult<Range, E>
    /// # where
    /// #     Q: Ord,
    /// #     V: Borrow<Q>,
    /// # {
    /// #     if unimplemented!() {
    /// slice.subslice_binary_search_by_key(target_value, locate_item_in)
    /// #     } else {
    /// // is equivalent to:
    /// subslice::binary_search_by_key(target_value, slice, locate_item_in)
    /// #     }
    /// # }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    fn subslice_binary_search_by_key<'a, V, Q, E>(
        &'a self,
        target_value: &Q,
        locate_item_in: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
    ) -> SearchResult<Range, E>
    where
        T: 'a,
        Q: Ord + ?Sized,
        V: Borrow<Q>,
    {
        subslice::binary_search_by_key(target_value, self.as_ref(), locate_item_in)
    }

    /// The function [`subslice::binary_search_by`] as an extension method.
    ///
    /// ```
    /// # use binary_search_collection::{subslice, Comparison, Range, SearchResult};
    /// # use binary_search_collection::ext::SliceExt as _;
    /// # fn f<T: Ord, E>(
    /// #     slice: &[T],
    /// #     mut locate_subslice_in: impl FnMut(&[T]) -> Result<Option<Comparison>, E> + Copy,
    /// # ) -> SearchResult<Range, E> {
    /// #     if unimplemented!() {
    /// slice.subslice_binary_search_by(locate_subslice_in)
    /// #     } else {
    /// // is equivalent to:
    /// subslice::binary_search_by(slice, locate_subslice_in)
    /// #     }
    /// # }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    fn subslice_binary_search_by<'a, E>(
        &'a self,
        locate_and_compare_in: impl FnMut(&'a [T]) -> Result<Option<Comparison>, E>,
    ) -> SearchResult<Range, E>
    where
        T: 'a,
    {
        subslice::binary_search_by(self.as_ref(), locate_and_compare_in)
    }

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

/// Extension methods for [slice]s.
pub trait ByteSliceExt: SliceExt<u8> {
    /// The function [`line::binary_search`] as an extension method.
    ///
    /// ```
    /// # use binary_search_collection::Range;
    /// # use binary_search_collection::ext::ByteSliceExt as _;
    /// # use binary_search_collection::line;
    /// # fn f(
    /// #     slice: &[u8],
    /// #     line: impl AsRef<[u8]>,
    /// # ) -> Result<Range, usize> {
    /// #     if unimplemented!() {
    /// slice.line_binary_search(line)
    /// #     } else {
    /// // is equivalent to:
    /// line::binary_search(line, slice)
    /// #     }
    /// # }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    fn line_binary_search(&self, line: impl AsRef<[u8]>) -> Result<Range, usize> {
        line::binary_search(line, self.as_ref())
    }

    /// The function [`line::binary_search_by_key`] as an extension method.
    ///
    /// ```
    /// # use binary_search_collection::{Range, SearchResult};
    /// # use binary_search_collection::ext::ByteSliceExt as _;
    /// # use binary_search_collection::line::{self, LineBytes};
    /// # use std::borrow::Borrow;
    /// # use std::cmp::Ordering;
    /// # fn f<'a, V, Q, E>(
    /// #     slice: &'a[u8],
    /// #     target_value: &Q,
    /// #     compare: impl FnMut(&mut LineBytes<'a>) -> Result<V, E>,
    /// # ) -> SearchResult<Range, E>
    /// # where
    /// #     Q: Ord,
    /// #     V: Borrow<Q>,
    /// # {
    /// #     if unimplemented!() {
    /// slice.line_binary_search_by_key(target_value, compare)
    /// #     } else {
    /// // is equivalent to:
    /// line::binary_search_by_key(target_value, slice, compare)
    /// #     }
    /// # }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    fn line_binary_search_by_key<'a, V, Q, E>(
        &'a self,
        target_value: &Q,
        extract: impl FnMut(&mut LineBytes<'a>) -> Result<V, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        V: Borrow<Q>,
    {
        line::binary_search_by_key(target_value, self.as_ref(), extract)
    }

    /// The function [`line::binary_search_by`] as an extension method.
    ///
    /// ```
    /// # use binary_search_collection::{Range, SearchResult};
    /// # use binary_search_collection::ext::ByteSliceExt as _;
    /// # use binary_search_collection::line::{self, LineBytes};
    /// # use std::cmp::Ordering;
    /// # fn f<'a, E>(
    /// #     slice: &'a[u8],
    /// #     compare: impl FnMut(&mut LineBytes<'a>) -> Result<Ordering, E>,
    /// # ) -> SearchResult<Range, E> {
    /// #     if unimplemented!() {
    /// slice.line_binary_search_by(compare)
    /// #     } else {
    /// // is equivalent to:
    /// line::binary_search_by(slice, compare)
    /// #     }
    /// # }
    /// ```
    #[expect(clippy::missing_errors_doc)]
    fn line_binary_search_by<'a, E>(
        &'a self,
        compare: impl FnMut(&mut LineBytes<'a>) -> Result<Ordering, E>,
    ) -> SearchResult<Range, E> {
        line::binary_search_by(self.as_ref(), compare)
    }

    /// Locates the line start delimited by LF from the slice end.
    ///
    /// Returns [`None`] if LF is not found.
    fn locate_line_start_from_end(&self) -> Option<usize> {
        self.as_ref().locate_last(|&b| b == LF).map(|i| i + 1)
    }

    /// Locates the line end delimited by LF from the slice start.
    ///
    /// Returns [`None`] if LF is not found.
    fn locate_line_end_from_start(&self) -> Option<LineEnd> {
        let slice = self.as_ref();
        slice.locate_first(|&b| b == LF).map(|lf| {
            if let Some(before_lf) = lf.checked_sub(1)
                && slice[before_lf] == CR
            {
                // CR + LF
                LineEnd {
                    position: before_lf,
                    line_break_len: 2,
                }
            } else {
                // LF
                LineEnd {
                    position: lf,
                    line_break_len: 1,
                }
            }
        })
    }

    /// Locates the line start from the slice end.
    ///
    /// Returns `0` if LF is not found.
    fn locate_line_start_from_end_or_zero(&self) -> usize {
        self.locate_line_start_from_end().unwrap_or(0)
    }

    /// Locates the line end from the slice start.
    ///
    /// Returns [`LineEnd`] with [`position`](LineEnd::position) equal to `self.len()` if LF is not
    /// found.
    fn locate_line_end_from_start_or_len(&self) -> LineEnd {
        let slice = self.as_ref();
        slice.locate_line_end_from_start().unwrap_or(LineEnd {
            position: slice.len(),
            line_break_len: 0,
        })
    }
}

impl ByteSliceExt for [u8] {}

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

impl<O: Offset, E> Offset for Result<O, E> {
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
