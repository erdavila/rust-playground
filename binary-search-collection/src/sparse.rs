//! Generic binary search.

use core::borrow::Borrow;
use core::cmp::Ordering;

use crate::ext::RangeExt as _;
use crate::{Comparison, LocatedItem, Range, SearchResult, sparse};

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
pub fn binary_search_by_key<T, Q, E>(
    target: &Q,
    element_count: usize,
    mut locate_item_in: impl FnMut(Range) -> Result<Option<LocatedItem<T>>, E>,
) -> SearchResult<Range, E>
where
    T: Borrow<Q>,
    Q: Ord + ?Sized,
{
    sparse::binary_search_by(element_count, |search_range| {
        let opt = locate_item_in(search_range)?.map(|li| match li.value.borrow().cmp(target) {
            Ordering::Less => Comparison::After(li.consumed_range.end),
            Ordering::Equal => Comparison::Found(li.value_range),
            Ordering::Greater => Comparison::Before(li.consumed_range.start),
        });

        Ok(opt)
    })
}

/// Executes a binary search in a sequence of _elements_ in an unspecified source with a custom
/// comparison.
///
/// Read the [crate root](self) documentation for basic information.
///
/// The `locate_and_compare_in` closure must locate an _item_ in the source _elements_ as delimited
/// by its range parameter, and decide if the _item_ is the wanted one, or if the search must
/// continue before or after it. **The search must start on the midpoint of the range towards both
/// ends.** If the search were linear in the range, then it would turn the entire binary search into
/// a linear search, defeating its purpose.
///
/// When the _item_ is found, the function returns [`Result::Ok`] with the range where it was found.
/// If the _item_ is NOT found, the function returns [`Result::Err`] with the index at which the
/// _item_ could be inserted in the source _elements_ while maintaining the sort order.
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by<E>(
    element_count: usize,
    mut locate_and_compare_in: impl FnMut(Range) -> Result<Option<Comparison>, E>,
) -> SearchResult<Range, E> {
    let mut start = 0;
    let mut end = element_count;

    while start < end {
        let search_range = (start..end).into();
        let Some(cmp) = locate_and_compare_in(search_range)? else {
            break;
        };

        match cmp {
            Comparison::Before(i) => {
                debug_assert!(i >= start);
                debug_assert!(i < end);
                end = i;
            }
            Comparison::Found(range) => {
                debug_assert!(range.is_subrange_of(search_range));
                return Ok(Ok(range));
            }
            Comparison::After(i) => {
                debug_assert!(i > start);
                debug_assert!(i <= end);
                start = i;
            }
        }
    }

    Ok(Err(start))
}
