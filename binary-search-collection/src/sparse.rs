//! Generic binary search.
//!
//! Read the [crate root](crate) documentation for information common to all functions in the crate.
//!
//! The functions in this modules perform search in an unspecified data source. It can be data from
//! memory, file, network, etc.
//!
//! The `limit` parameter in the functions delimits where the search is performed in the source.

use core::borrow::Borrow;
use core::cmp::Ordering;

use crate::ext::RangeExt as _;
use crate::{Comparison, LocatedItem, Range, SearchResult, sparse};

/// Executes a binary search in an unspecified source with a comparison key extraction function.
///
/// Read the [module](self) documentation for information common to all functions in the module.
///
/// The `locate` closure must [locate](crate#locate) the value in the source as delimited by its
/// [`Range`] parameter.
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by_key<T, Q, E>(
    target: &Q,
    limit: usize,
    mut locate: impl FnMut(Range) -> Result<Option<LocatedItem<T>>, E>,
) -> SearchResult<Range, E>
where
    T: Borrow<Q>,
    Q: Ord + ?Sized,
{
    sparse::binary_search_by(limit, |search_range| {
        let opt = locate(search_range)?.map(|li| match li.value.borrow().cmp(target) {
            Ordering::Less => Comparison::After(li.consumed_range.end),
            Ordering::Equal => Comparison::Found(li.value_range),
            Ordering::Greater => Comparison::Before(li.consumed_range.start),
        });

        Ok(opt)
    })
}

/// Executes a binary search in an unspecified source with a custom comparison.
///
/// Read the [module](self) documentation for information common to all functions in the module.
///
/// The `locate_and_compare` closure must evaluate the [located](crate#locate) value and return a
/// [`Comparison`] that will drive the remaining of the search.
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by<E>(
    limit: usize,
    mut locate_and_compare: impl FnMut(Range) -> Result<Option<Comparison>, E>,
) -> SearchResult<Range, E> {
    let mut start = 0;
    let mut end = limit;

    while start < end {
        let search_range = (start..end).into();
        let Some(cmp) = locate_and_compare(search_range)? else {
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
