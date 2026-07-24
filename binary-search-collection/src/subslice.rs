//! Binary search in a slice or array.

use crate::ext::{Offset, RangeExt as _, SliceExt as _};
use crate::{LocatedItem, Range, SearchResult, sparse};

/// The location of a subslice in a source slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocatedSubslice {
    /// The range of the source slice that delimits the _elements_ of the subslice.
    ///
    /// It must be a subrange of `consumed_range`.
    pub subslice_range: Range,

    /// The range of _element_ in the source slice that were "consumed" in the search for the subslice.
    ///
    /// It may include delimiters that were used to determine the `subslice_range`.
    pub consumed_range: Range,
}

impl Offset for LocatedSubslice {
    fn offset(self, amount: usize) -> Self {
        LocatedSubslice {
            subslice_range: self.subslice_range.offset(amount),
            consumed_range: self.consumed_range.offset(amount),
        }
    }
}

/// Executes a binary search for a `subslice` in a `source` slice.
///
/// Read the [module](self) documentation for basic information.
///
/// The `locate_subslice_in` closure must locate a subslice in the source _elements_ as delimited by
/// its range parameter. **The search must start on the midpoint of the range towards both ends.**
/// If the search were linear in the range, then it would turn the entire binary search into a
/// linear search, defeating its purpose.
///
/// If the [`subslice_range`](LocatedSubslice::subslice_range) in the [`LocatedSubslice`] returned
/// by `locate_subslice_in` is not the subslice that is being searched for, the binary search will
/// continue in either before or after the [`consumed_range`](LocatedSubslice::consumed_range).
///
/// When the `target_subslice` is found, the function returns [`Result::Ok`] with the corresponding
/// [`LocatedSubslice`]. If the `target_subslice` is NOT found, the function returns [`Result::Err`]
/// with the index at which the `target_subslice` could be inserted in the `source` _elements_ while
/// maintaining the sort order.
///
/// # Examples
///
/// ## Delimited subslices
///
/// When a delimiter is not expected to be followed by another one:
///
/// ```
/// use binary_search_collection::{Range, SearchResult};
/// use binary_search_collection::subslice::{self, LocatedSubslice};
/// use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
/// use std::assert_matches;
///
/// fn find_delimited(sequence: &[char], chars: &[char], delimiter: char) -> SearchResult<Range> {
///     subslice::binary_search(sequence, chars, |slice| {
///         let located_subslice = slice.subslice_range_from_midpoint_to_delimiters(|&c| c == delimiter);
///         Some(located_subslice)
///     })
/// }
///
/// let chars = ['b', 'b', '-', 'd', 'd', '-', 'f', 'f'];
///
/// assert_eq!(     find_delimited(&['a', 'a'], &chars, '-'), Err(0));
/// assert_eq!(     find_delimited(&['b', 'b'], &chars, '-'), Ok((0..2).into()));
/// assert_matches!(find_delimited(&['c', 'c'], &chars, '-'), Err(2..=3));
/// assert_eq!(     find_delimited(&['d', 'd'], &chars, '-'), Ok((3..5).into()));
/// assert_matches!(find_delimited(&['e', 'e'], &chars, '-'), Err(5..=6));
/// assert_eq!(     find_delimited(&['f', 'f'], &chars, '-'), Ok((6..8).into()));
/// assert_eq!(     find_delimited(&['g', 'g'], &chars, '-'), Err(8));
/// ```
///
/// ## Subslices separated by gaps
///
/// When multiple delimiters in sequence constitute a gap:
///
/// ```
/// use binary_search_collection::{subslice, Range, SearchResult};
/// use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
/// use std::assert_matches;
///
/// fn find_subslice_with_gaps<T: Ord>(
///     subslice: &[T],
///     source: &[T],
///     gap: &T,
/// ) -> SearchResult<Range> {
///     subslice::binary_search(subslice, source, |slice| {
///         slice
///             .locate_from_midpoint(|x| x != gap)
///             .map(|non_gap_index| {
///                 let non_gap_range = Range::from_start_and_len(non_gap_index, 1);
///                 slice.extend_subslice_range_to_delimiters(non_gap_range, |x| x == gap)
///             })
///     })
/// }
///
/// let elements = ['a', 'a', '-', '-', '-', 'b', 'b', '-', 'c', 'c', '-', '-'];
///
/// assert_eq!(     find_subslice_with_gaps(&['a'], &elements, &'-'), Err(0));
/// assert_eq!(     find_subslice_with_gaps(&['a', 'a'], &elements, &'-'), Ok((0..2).into()));
/// assert_matches!(find_subslice_with_gaps(&['b'], &elements, &'-'), Err(2..=5));
/// assert_eq!(     find_subslice_with_gaps(&['b', 'b'], &elements, &'-'), Ok((5..7).into()));
/// assert_matches!(find_subslice_with_gaps(&['c'], &elements, &'-'), Err(7..=8));
/// assert_eq!(     find_subslice_with_gaps(&['c', 'c'], &elements, &'-'), Ok((8..10).into()));
/// assert_matches!(find_subslice_with_gaps(&['d'], &elements, &'-'), Err(10..=12));
/// ```
///
/// ## Regular binary search
///
/// Just like [`slice::binary_search`]:
///
/// ```
/// use binary_search_collection::{Range, SearchResult};
/// use binary_search_collection::subslice::{self, LocatedSubslice};
/// use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
///
/// fn regular_binary_search<T: Ord>(value: &T, list: &[T]) -> SearchResult<usize> {
///     let value_slice = core::slice::from_ref(value);
///     let result = subslice::binary_search(value_slice, list, |slice| {
///         let value_range = Range::from_start_and_len(slice.range().midpoint(), 1);
///         Some(LocatedSubslice {
///             subslice_range: value_range,
///             consumed_range: value_range,
///         })
///     });
///
///     result.map(|range| range.start)
/// }
///
/// assert_eq!(regular_binary_search(&5, &[10, 20, 30]), Err(0));
/// assert_eq!(regular_binary_search(&10, &[10, 20, 30]), Ok(0));
/// assert_eq!(regular_binary_search(&15, &[10, 20, 30]), Err(1));
/// assert_eq!(regular_binary_search(&20, &[10, 20, 30]), Ok(1));
/// assert_eq!(regular_binary_search(&25, &[10, 20, 30]), Err(2));
/// assert_eq!(regular_binary_search(&30, &[10, 20, 30]), Ok(2));
/// assert_eq!(regular_binary_search(&35, &[10, 20, 30]), Err(3));
/// ```
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<'a, T: Ord>(
    target_subslice: &[T],
    source: &'a [T],
    mut locate_subslice_in: impl FnMut(&'a [T]) -> Option<LocatedSubslice>,
) -> SearchResult<Range> {
    sparse::binary_search(target_subslice, source.len(), |search_range| {
        source
            .in_range(search_range, &mut locate_subslice_in)
            .map(|ls| {
                debug_assert!(ls.subslice_range.is_subrange_of(ls.consumed_range));
                debug_assert!(ls.consumed_range.is_subrange_of(search_range));

                let value_range = ls.subslice_range;

                LocatedItem {
                    value: &source[value_range],
                    value_range,
                    consumed_range: ls.consumed_range,
                }
            })
    })
}
