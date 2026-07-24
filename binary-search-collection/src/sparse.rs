//! Generic binary search.

use core::borrow::Borrow;
use core::cmp::Ordering;

use crate::ext::RangeExt as _;
use crate::{LocatedItem, Range, SearchResult};

/// Executes a binary search in a sequence of _elements_ in an unspecified source.
///
/// Read the [module](self) documentation for basic information.
///
/// The `locate_item_in` closure must locate an _item_ in the source _elements_ as delimited by its
/// range parameter. **The search must start on the midpoint of the range towards both ends.** If
/// the search were linear in the range, then it would turn the entire binary search into a linear
/// search, defeating its purpose.
///
/// If the [`value`](LocatedItem::value) in the [`LocatedItem`] returned by `locate_item_in` is not
/// the _item_ that is being search for, the binary search will continue in either before or after
/// the [`consumed_range`](LocatedItem::consumed_range).
///
/// When the `target` is found, the function returns [`Result::Ok`] with the range where the value
/// was found. If the `target` is NOT found, the function returns [`Result::Err`] with the index at
/// which the `target` could be inserted in the source _elements_ while maintaining the sort order.
///
/// # Example
///
/// Subslices with delimiters, sorted in reverse order:
///
/// ```
/// use binary_search_collection::{sparse, LocatedItem, Range, SearchResult};
/// use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
/// use std::assert_matches;
/// use std::cmp::Ordering;
///
/// fn reverse_find_delimited(sequence: &[char], chars: &[char], delimiter: char) -> SearchResult<Range> {
///     #[derive(PartialEq, Eq)]
///     struct Reverse<T>(T);
///
///     impl<T: Ord> PartialOrd for Reverse<T> {
///         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///             Some(self.cmp(other))
///         }
///     }
///
///     impl<T: Ord> Ord for Reverse<T> {
///         fn cmp(&self, other: &Self) -> Ordering {
///             self.0.cmp(&other.0).reverse()
///         }
///     }
///
///     sparse::binary_search(&Reverse(sequence), chars.len(), |range| {
///         let located_subslice = chars
///             .in_range(range, |slice| {
///                 slice.subslice_range_from_midpoint_to_delimiters(|&c| c == delimiter)
///             });
///
///         let value_range = located_subslice.subslice_range;
///         let value = Reverse(&chars[value_range]);
///         let consumed_range = located_subslice.consumed_range;
///
///         Some(LocatedItem {
///             value,
///             value_range,
///             consumed_range,
///         })
///     })
/// }
///
/// let chars = ['f', 'f', '-', 'd', 'd', '-', 'b', 'b'];
///
/// assert_eq!(     reverse_find_delimited(&['g', 'g'], &chars, '-'), Err(0));
/// assert_eq!(     reverse_find_delimited(&['f', 'f'], &chars, '-'), Ok((0..2).into()));
/// assert_matches!(reverse_find_delimited(&['e', 'e'], &chars, '-'), Err(2..=3));
/// assert_eq!(     reverse_find_delimited(&['d', 'd'], &chars, '-'), Ok((3..5).into()));
/// assert_matches!(reverse_find_delimited(&['c', 'c'], &chars, '-'), Err(5..=6));
/// assert_eq!(     reverse_find_delimited(&['b', 'b'], &chars, '-'), Ok((6..8).into()));
/// assert_eq!(     reverse_find_delimited(&['a', 'a'], &chars, '-'), Err(8));
/// ```
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<T, Q>(
    target: &Q,
    element_count: usize,
    mut locate_item_in: impl FnMut(Range) -> Option<LocatedItem<T>>,
) -> SearchResult<Range>
where
    T: Borrow<Q>,
    Q: Ord + ?Sized,
{
    let mut start = 0;
    let mut end = element_count;

    while start < end {
        let search_range = (start..end).into();
        let Some(li) = locate_item_in(search_range) else {
            break;
        };

        debug_assert!(li.consumed_range.is_subrange_of(search_range));

        match li.value.borrow().cmp(target) {
            Ordering::Less => start = li.consumed_range.end,
            Ordering::Equal => return Ok(li.value_range),
            Ordering::Greater => end = li.consumed_range.start,
        }
    }

    Err(start)
}
