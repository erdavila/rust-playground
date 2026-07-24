//! Generic binary search.

use core::borrow::Borrow;
use core::cmp::Ordering;

use crate::ext::RangeExt as _;
use crate::{LocatedItem, Range, SearchResult};

/// Executes a binary search in a sequence of _elements_ in an unspecified source.
///
/// Read the [crate root](self) documentation for basic information.
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
/// - [Values with gaps](crate#values-with-gaps)
/// - [Subslices with delimiters with a custom comparison](crate#subslices-with-delimiters-with-a-custom-comparison)
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<T, Q, E>(
    target: &Q,
    element_count: usize,
    mut locate_item_in: impl FnMut(Range) -> Result<Option<LocatedItem<T>>, E>,
) -> SearchResult<Range, E>
where
    T: Borrow<Q>,
    Q: Ord + ?Sized,
{
    let mut start = 0;
    let mut end = element_count;

    while start < end {
        let search_range = (start..end).into();
        let Some(li) = locate_item_in(search_range)? else {
            break;
        };

        debug_assert!(li.consumed_range.is_subrange_of(search_range));

        match li.value.borrow().cmp(target) {
            Ordering::Less => start = li.consumed_range.end,
            Ordering::Equal => return Ok(Ok(li.value_range)),
            Ordering::Greater => end = li.consumed_range.start,
        }
    }

    Ok(Err(start))
}
