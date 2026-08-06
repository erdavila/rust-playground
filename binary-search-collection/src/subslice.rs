//! Binary search in a slice or array.
//!
//! Read the [crate root](crate) documentation for information common to all functions in the crate.
//!
//! The functions are also available as [extension method for slices](crate::ext::SliceExt).

use core::borrow::Borrow;

use crate::ext::{Offset, SliceExt as _};
use crate::{Comparison, LocatedItem, Range, SearchResult, sparse};

/// The location of a subslice in a source slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocatedSubslice {
    /// The range in the source slice that delimits the located value.
    ///
    /// It must be a subrange of `consumed_range`.
    pub subslice_range: Range,

    /// The range in the source slice that was "consumed" in the search for the subslice.
    ///
    /// It may include delimiters that were used to determine the `subslice_range`, and is used to
    /// delimit the resuming search when the located subslice does not correspond to the target.
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

/// Executes a binary search for a subslice in a `source` slice.
///
/// Read the [module](self) documentation for information common to all functions in the module.
///
/// The `locate` closure must [locate](crate#locate) a subslice in its slice parameter to be
/// compared to the `target` slice.
///
/// This function is also available as an [extension method for
/// slices](crate::ext::SliceExt::subslice_binary_search).
///
/// # Examples
///
/// - [Subslices with gaps](crate#subslices-with-gaps)
/// - [Subslices with delimiters](crate#subslices-with-delimiters)
/// - [Regular binary search](crate#regular-binary-search)
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<'a, T: Ord, E>(
    target: &[T],
    source: &'a [T],
    locate: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
) -> SearchResult<Range, E> {
    binary_search::implementation(target, source, locate)
}

// Implementation using `subslice::binary_search_by`.
#[cfg(not(feature = "alternative-subslice-binary_search"))]
mod binary_search {
    use crate::subslice::{self, LocatedSubslice};
    use crate::{LocatedItem, Range, SearchResult};

    pub(super) fn implementation<'a, T: Ord, E>(
        target: &[T],
        source: &'a [T],
        mut locate: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
    ) -> SearchResult<Range, E> {
        subslice::binary_search_by_key(target, source, |search_slice| {
            let li = locate(search_slice)?.map(|ls| {
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
        target: &[T],
        source: &'a [T],
        mut locate: impl FnMut(&'a [T]) -> Result<Option<LocatedSubslice>, E>,
    ) -> SearchResult<Range, E> {
        sparse::binary_search_by_key(target, source.len(), |search_range| {
            let loc_item_opt = source.in_range(search_range, &mut locate)?.map(|ls| {
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

/// Executes a binary search for a subslice in a `source` slice with a comparison key extraction
/// function.
///
/// Read the [module](self) documentation for information common to all functions in the module.
///
/// The `locate` closure must [locate](crate#locate) a subslice in its slice parameter a return a
/// value to be compared to the `target`.
///
/// This function is also available as an [extension method for
/// slices](crate::ext::SliceExt::subslice_binary_search_by_key).
///
/// # Example
///
/// - [Subslices with delimiters with a custom
///   comparison](crate#subslices-with-delimiters-with-a-custom-comparison)
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by_key<'a, T, V, Q, E>(
    target: &Q,
    source: &'a [T],
    locate: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
) -> SearchResult<Range, E>
where
    Q: Ord + ?Sized,
    V: Borrow<Q>,
{
    binary_search_by_key::implementation(target, source, locate)
}

// Implementation using `subslice::binary_search_by`.
#[cfg(not(feature = "alternative-subslice-binary_search_by_key"))]
mod binary_search_by_key {
    use core::borrow::Borrow;
    use core::cmp::Ordering;

    use crate::{Comparison, LocatedItem, Range, SearchResult, subslice};

    pub(super) fn implementation<'a, T, V, Q, E>(
        target: &Q,
        source: &'a [T],
        mut locate: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        V: Borrow<Q>,
    {
        subslice::binary_search_by(source, |search_slice| {
            let cmp = locate(search_slice)?.map(|li| match li.value.borrow().cmp(target) {
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
        target: &Q,
        source: &'a [T],
        mut locate: impl FnMut(&'a [T]) -> Result<Option<LocatedItem<V>>, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        V: Borrow<Q>,
    {
        sparse::binary_search_by_key(target, source.len(), |search_range| {
            source.in_range(search_range, &mut locate)
        })
    }
}

/// Executes a binary search for a subslice in a `source` slice with a custom comparison.
///
/// Read the [module](self) documentation for information common to all functions in the module.
///
/// The `locate_and_compare` closure must evaluate the [located](crate#locate) subslice and return a
/// [`Comparison`] that will drive the remaining of the search.
///
/// This function is also available as an [extension method for
/// slices](crate::ext::SliceExt::subslice_binary_search_by).
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by<'a, T, E>(
    source: &'a [T],
    mut locate_and_compare: impl FnMut(&'a [T]) -> Result<Option<Comparison>, E>,
) -> SearchResult<Range, E> {
    sparse::binary_search_by(source.len(), |search_range| {
        source.in_range(search_range, &mut locate_and_compare)
    })
}
