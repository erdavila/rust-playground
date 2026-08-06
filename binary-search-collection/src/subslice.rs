//! Binary search in a slice or array.

use core::borrow::Borrow;

use crate::ext::{Offset, SliceExt as _};
use crate::{Comparison, LocatedItem, Range, SearchResult, sparse};

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
/// Read the [crate root](self) documentation for basic information.
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
/// This function is also available as an [extension method for slices](crate::ext::SliceExt::subslice_binary_search).
///
/// # Examples
///
/// - [Subslices with gaps](crate#subslices-with-gaps)
/// - [Subslices with delimiters](crate#subslices-with-delimiters)
/// - [Regular binary search](crate#regular-binary-search)
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<'a, T: Ord, E>(
    target_subslice: &[T],
    source: &'a [T],
    locate_subslice_in: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
) -> SearchResult<Range, E> {
    binary_search::implementation(target_subslice, source, locate_subslice_in)
}

// Implementation using `subslice::binary_search_by`.
#[cfg(not(feature = "alternative-subslice-binary_search"))]
mod binary_search {
    use crate::subslice::{self, LocatedSubslice};
    use crate::{LocatedItem, Range, SearchResult};

    pub(super) fn implementation<'a, T: Ord, E>(
        target_subslice: &[T],
        source: &'a [T],
        mut locate_subslice_in: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
    ) -> SearchResult<Range, E> {
        subslice::binary_search_by_key(target_subslice, source, |search_slice| {
            let li = locate_subslice_in(search_slice)?.map(|ls| {
                let value = &search_slice[ls.subslice_range];
                LocatedItem {
                    value,
                    value_range: ls.subslice_range,
                    consumed_range: ls.consumed_range,
                }
            });
            Ok(li)
        })
    }
}

// Implementation using `sparse::binary_search_by_key`.
#[cfg(feature = "alternative-subslice-binary_search")]
mod binary_search {
    use crate::ext::{RangeExt as _, SliceExt as _};
    use crate::subslice::LocatedSubslice;
    use crate::{LocatedItem, Range, SearchResult, sparse};

    pub(super) fn implementation<'a, T: Ord, E>(
        target_subslice: &[T],
        source: &'a [T],
        mut locate_subslice_in: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
    ) -> SearchResult<Range, E> {
        sparse::binary_search_by_key(target_subslice, source.len(), |search_range| {
            let loc_item_opt = source
                .in_range(search_range, &mut locate_subslice_in)?
                .map(|ls| {
                    debug_assert!(ls.subslice_range.is_subrange_of(ls.consumed_range));
                    debug_assert!(ls.consumed_range.is_subrange_of(search_range));

                    let value_range = ls.subslice_range;

                    LocatedItem {
                        value: &source[value_range],
                        value_range,
                        consumed_range: ls.consumed_range,
                    }
                });

            Ok(loc_item_opt)
        })
    }
}

/// Executes a binary search for a subslice in a `source` slice with a comparison key extraction.
///
/// Read the [crate root](self) documentation for basic information.
///
/// The `locate_item_in` closure must locate a subslice in the source _elements_ as delimited by its
/// slice parameter, and extract a value of type `V` to be compared against the `target_value`.
/// **The search must start on the midpoint of the range towards both ends.** If the search were
/// linear in the range, then it would turn the entire binary search into a linear search, defeating
/// its purpose.
///
/// When the subslice is found, the function returns [`Result::Ok`] with the range where it was found.
/// If the subslice is NOT found, the function returns [`Result::Err`] with the index at which the
/// subslice could be inserted in the `source` _elements_ while maintaining the sort order.
///
/// This function is also available as an [extension method for slices](crate::ext::SliceExt::subslice_binary_search_by_key).
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by_key<'a, T, V, Q, E>(
    target_value: &Q,
    source: &'a [T],
    locate_item_in: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
) -> SearchResult<Range, E>
where
    Q: Ord + ?Sized,
    V: Borrow<Q>,
{
    binary_search_by_key::implementation(target_value, source, locate_item_in)
}

// Implementation using `subslice::binary_search_by`.
#[cfg(not(feature = "alternative-subslice-binary_search_by_key"))]
mod binary_search_by_key {
    use core::borrow::Borrow;
    use core::cmp::Ordering;

    use crate::{Comparison, LocatedItem, Range, SearchResult, subslice};

    pub(super) fn implementation<'a, T, V, Q, E>(
        target_value: &Q,
        source: &'a [T],
        mut locate_item_in: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        V: Borrow<Q>,
    {
        subslice::binary_search_by(source, |search_slice| {
            let cmp =
                locate_item_in(search_slice)?.map(|li| match li.value.borrow().cmp(target_value) {
                    Ordering::Less => Comparison::After(li.consumed_range.end),
                    Ordering::Equal => Comparison::Found(li.value_range),
                    Ordering::Greater => Comparison::Before(li.consumed_range.start),
                });
            Ok(cmp)
        })
    }
}

// Implementation using `sparse::binary_search`.
#[cfg(feature = "alternative-subslice-binary_search_by_key")]
mod binary_search_by_key {
    use core::borrow::Borrow;

    use crate::ext::SliceExt as _;
    use crate::{LocatedItem, Range, SearchResult, sparse};

    pub(super) fn implementation<'a, T, V, Q, E>(
        target_value: &Q,
        source: &'a [T],
        mut locate_item_in: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        V: Borrow<Q>,
    {
        sparse::binary_search_by_key(target_value, source.len(), |search_range| {
            source.in_range(search_range, &mut locate_item_in)
        })
    }
}

/// Executes a binary search for a subslice in a `source` slice with a custom comparison.
///
/// Read the [crate root](self) documentation for basic information.
///
/// The `locate_and_compare_in` closure must locate a subslice in the source _elements_ as delimited by
/// its slice parameter, and decide if the _item_ is the wanted one, or if the search must continue
/// before or after it. **The search must start on the midpoint of the range towards both ends.** If
/// the search were linear in the range, then it would turn the entire binary search into a linear
/// search, defeating its purpose.
///
/// When the subslice is found, the function returns [`Result::Ok`] with the range where it was found.
/// If the subslice is NOT found, the function returns [`Result::Err`] with the index at which the
/// subslice could be inserted in the `source` _elements_ while maintaining the sort order.
///
/// This function is also available as an [extension method for slices](crate::ext::SliceExt::subslice_binary_search_by).
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by<'a, T, E>(
    source: &'a [T],
    mut locate_and_compare_in: impl FnMut(&'a [T]) -> Result<Option<Comparison>, E>,
) -> SearchResult<Range, E> {
    sparse::binary_search_by(source.len(), |search_range| {
        source.in_range(search_range, &mut locate_and_compare_in)
    })
}
