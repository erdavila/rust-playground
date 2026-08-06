#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Provides binary search functions for ranges where distinct positions may not correspond to
//! distinct values.
//!
//! This includes cases where some positions are ignored, or where values can span across multiple
//! positions. Also, the binary search may be run in an unspecified source, e.b. by loading data
//! on-demand.
//!
//! When the target value is found, the function returns [`Result::Ok`] with the range where the
//! value was found. If the value is NOT found, the function returns [`Result::Err`] with the index
//! at which the target value could be inserted in the source elements while maintaining the sort
//! order.
//!
//! # Function families<a id="families"></a>
//!
//! The functions are grouped into families, with each family corresponding to a module:
//!
//! - [`sparse`]: The most general functions. All other functions are defined in terms of these.
//! - [`subslice`]: Functions for searching subslices.
//! - [`line`]: Functions for searching text lines (delimited by line breaks) in in-memory data.
//! - [`line::buffered`]: Functions for searching text lines in [`File`]s.
//!
//! [`line`]: mod@line
//! [`File`]: std::fs::File
//!
//! # Functions variations
//!
//! Each [family](#families) has the following functions:
//!
//! - `binary_search`: Searches for a target value from the source data. (Not available in the
//!   [`sparse`] family.)
//! - `binary_search_by_key`: Searches for a target value by comparing to keys derived from the
//!   source data.
//! - `binary_search_by`: A custom comparison of values in the source data drives the search.
//!
//! # Locating values<a id="locate"></a>
//!
//! The [`sparse`] and [`subslice`] functions require a closure that locates a value in the source
//! as delimited by its [`Range`] or slice parameter. **The search must start at the midpoint of the
//! range towards both ends.** If the search were linear in the range, then it would turn the entire
//! binary search into a linear search, defeating its purpose.
//!
//! If there is not a value in the range, then the closure must return [`None`].
//!
//! To locate the value in the source, consider using the
//! [`iter_from_midpoint`](crate::ext::RangeExt::iter_from_midpoint) extension method to iterate on
//! the positions from the midpoint. For [`subslice`] search, also consider the
//! [`extend_subslice_range_to_delimiters`](ext::SliceExt::extend_subslice_range_to_delimiters) and
//! [`subslice_range_from_midpoint_to_delimiters`](ext::SliceExt::subslice_range_from_midpoint_to_delimiters)
//! extension methods.
//!
//! The `consumed_range` field in the returned [`LocatedItem`](LocatedItem::consumed_range) or
//! [`LocatedSubslice`](subslice::LocatedSubslice::consumed_range) is used to delimit the resuming
//! search when the located value does not correspond to the target.
//!
//! # Cases and Examples
//!
//! ## Values with gaps<a id="values-with-gaps"></a>
//!
//! A list of values where some of them must be ignored.
//!
//! Using [`subslice::binary_search_by_key`]:
//!
//! ```
//! use std::assert_matches;
//! use std::convert::Infallible;
//!
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use binary_search_collection::{LocatedItem, Range, subslice};
//!
//! fn binary_search_with_gaps<T, V: Ord>(
//!     target: &V,
//!     list: &[T],
//!     mut extract: impl FnMut(&T) -> Option<V>,
//! ) -> Result<usize, usize> {
//!     let Ok(result) =
//!         subslice::binary_search_by_key::<_, _, _, Infallible>(target, list, |search_slice| {
//!             let located = search_slice.range().iter_from_midpoint().find_map(|i| {
//!                 extract(&search_slice[i]).map(|value| {
//!                     let range = Range::from_start_and_len(i, 1);
//!                     LocatedItem {
//!                         value,
//!                         value_range: range,
//!                         consumed_range: range,
//!                     }
//!                 })
//!             });
//!             Ok(located)
//!         });
//!
//!     result.map(|range| range.start)
//! }
//!
//! let list = [
//!     Ok(10),
//!     Ok(20),
//!     Err("ignored"),
//!     Err("not found"),
//!     Ok(30),
//!     Err("failed"),
//! ];
//!
//! assert_eq!(
//!     binary_search_with_gaps(&5, &list, |result| result.ok()),
//!     Err(0)
//! );
//! assert_eq!(
//!     binary_search_with_gaps(&10, &list, |result| result.ok()),
//!     Ok(0)
//! );
//! assert_eq!(
//!     binary_search_with_gaps(&15, &list, |result| result.ok()),
//!     Err(1)
//! );
//! assert_eq!(
//!     binary_search_with_gaps(&20, &list, |result| result.ok()),
//!     Ok(1)
//! );
//! assert_matches!(
//!     binary_search_with_gaps(&25, &list, |result| result.ok()),
//!     Err(2..=4)
//! );
//! assert_eq!(
//!     binary_search_with_gaps(&30, &list, |result| result.ok()),
//!     Ok(4)
//! );
//! assert_matches!(
//!     binary_search_with_gaps(&35, &list, |result| result.ok()),
//!     Err(5..=6)
//! );
//! ```
//!
//! ## Subslices with delimiters<a id="subslices-with-delimiters"></a>
//!
//! A list of values that span across the source elements, separated by a single delimiter element.
//!
//! Using [`subslice::binary_search`]:
//!
//! ```
//! use std::assert_matches;
//! use std::convert::Infallible;
//!
//! use binary_search_collection::ext::SliceExt as _;
//! use binary_search_collection::{Range, subslice};
//!
//! fn find_delimited(target: &[char], source: &[char], delimiter: char) -> Result<Range, usize> {
//!     let Ok(result) = subslice::binary_search::<_, Infallible>(target, source, |search_slice| {
//!         let located =
//!             search_slice.subslice_range_from_midpoint_to_delimiters(|&c| c == delimiter);
//!         Ok(Some(located))
//!     });
//!
//!     result
//! }
//!
//! const GAP: char = '-';
//! let chars = ['b', 'b', GAP, 'd', 'd', GAP, 'f', 'f'];
//!
//! assert_eq!(find_delimited(&['a', 'a'], &chars, GAP), Err(0));
//! assert_eq!(find_delimited(&['b', 'b'], &chars, GAP), Ok((0..2).into()));
//! assert_matches!(find_delimited(&['c', 'c'], &chars, GAP), Err(2..=3));
//! assert_eq!(find_delimited(&['d', 'd'], &chars, GAP), Ok((3..5).into()));
//! assert_matches!(find_delimited(&['e', 'e'], &chars, GAP), Err(5..=6));
//! assert_eq!(find_delimited(&['f', 'f'], &chars, GAP), Ok((6..8).into()));
//! assert_eq!(find_delimited(&['g', 'g'], &chars, GAP), Err(8));
//! ```
//!
//! ## Subslices with gaps<a id="subslices-with-gaps"></a>
//!
//! A list of values that span across the source elements, separated by delimiting elements that may
//! occupy multiple consecutive positions.
//!
//! Using [`subslice::binary_search`]:
//!
//! ```
//! use std::assert_matches;
//! use std::convert::Infallible;
//!
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use binary_search_collection::{Range, subslice};
//!
//! fn find_subslice_with_gaps<T: Ord>(
//!     target: &[T],
//!     source: &[T],
//!     gap: &T,
//! ) -> Result<Range, usize> {
//!     let Ok(result) = subslice::binary_search::<_, Infallible>(target, source, |search_slice| {
//!         let located = search_slice
//!             .locate_from_midpoint(|x| x != gap)
//!             .map(|non_gap_index| {
//!                 let non_gap_range = Range::from_start_and_len(non_gap_index, 1);
//!                 search_slice.extend_subslice_range_to_delimiters(non_gap_range, |x| x == gap)
//!             });
//!         Ok(located)
//!     });
//!
//!     result
//! }
//!
//! const GAP: char = '-';
//! let source = ['a', 'a', GAP, GAP, GAP, 'b', 'b', GAP, 'c', 'c', GAP, GAP];
//!
//! assert_eq!(find_subslice_with_gaps(&['a'], &source, &GAP), Err(0));
//! assert_eq!(
//!     find_subslice_with_gaps(&['a', 'a'], &source, &GAP),
//!     Ok((0..2).into())
//! );
//! assert_matches!(find_subslice_with_gaps(&['b'], &source, &GAP), Err(2..=5));
//! assert_eq!(
//!     find_subslice_with_gaps(&['b', 'b'], &source, &GAP),
//!     Ok((5..7).into())
//! );
//! assert_matches!(find_subslice_with_gaps(&['c'], &source, &GAP), Err(7..=8));
//! assert_eq!(
//!     find_subslice_with_gaps(&['c', 'c'], &source, &GAP),
//!     Ok((8..10).into())
//! );
//! assert_matches!(find_subslice_with_gaps(&['d'], &source, &GAP), Err(10..=12));
//! ```
//!
//! ## Subslices with delimiters with a custom comparison<a id="subslices-with-delimiters-with-a-custom-comparison"></a>
//!
//! A custom comparison can be used with [`subslice::binary_search_by_key`].
//!
//! In the first example, slices are compared in reverse order:
//!
//! ```
//! use std::assert_matches;
//! use std::cmp::Ordering;
//! use std::convert::Infallible;
//!
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use binary_search_collection::{LocatedItem, Range, subslice};
//!
//! fn reverse_find_delimited(
//!     target: &[char],
//!     source: &[char],
//!     delimiter: char,
//! ) -> Result<Range, usize> {
//!     #[derive(PartialEq, Eq)]
//!     struct Reverse<T>(T);
//!
//!     impl<T: Ord> PartialOrd for Reverse<T> {
//!         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!             Some(self.cmp(other))
//!         }
//!     }
//!
//!     impl<T: Ord> Ord for Reverse<T> {
//!         fn cmp(&self, other: &Self) -> Ordering {
//!             self.0.cmp(&other.0).reverse()
//!         }
//!     }
//!
//!     let Ok(result) = subslice::binary_search_by_key::<_, _, _, Infallible>(
//!         &Reverse(target),
//!         source,
//!         |search_slice| {
//!             let located =
//!                 search_slice.subslice_range_from_midpoint_to_delimiters(|&c| c == delimiter);
//!             let value_range = located.subslice_range;
//!
//!             Ok(Some(LocatedItem {
//!                 value: Reverse(&search_slice[value_range]),
//!                 value_range,
//!                 consumed_range: located.consumed_range,
//!             }))
//!         },
//!     );
//!
//!     result
//! }
//!
//! const DELIMITER: char = '-';
//! let chars = ['f', 'f', DELIMITER, 'd', 'd', DELIMITER, 'b', 'b'];
//!
//! assert_eq!(
//!     reverse_find_delimited(&['g', 'g'], &chars, DELIMITER),
//!     Err(0)
//! );
//! assert_eq!(
//!     reverse_find_delimited(&['f', 'f'], &chars, DELIMITER),
//!     Ok((0..2).into())
//! );
//! assert_matches!(
//!     reverse_find_delimited(&['e', 'e'], &chars, DELIMITER),
//!     Err(2..=3)
//! );
//! assert_eq!(
//!     reverse_find_delimited(&['d', 'd'], &chars, DELIMITER),
//!     Ok((3..5).into())
//! );
//! assert_matches!(
//!     reverse_find_delimited(&['c', 'c'], &chars, DELIMITER),
//!     Err(5..=6)
//! );
//! assert_eq!(
//!     reverse_find_delimited(&['b', 'b'], &chars, DELIMITER),
//!     Ok((6..8).into())
//! );
//! assert_eq!(
//!     reverse_find_delimited(&['a', 'a'], &chars, DELIMITER),
//!     Err(8)
//! );
//! ```
//!
//! In the second example, a search is performed in a comma-delimited numbers list:
//!
//! ```
//! use std::assert_matches;
//! use std::num::ParseIntError;
//! use std::str::Utf8Error;
//!
//! use binary_search_collection::ext::SliceExt as _;
//! use binary_search_collection::{LocatedItem, Range, SearchResult, subslice};
//!
//! #[derive(Debug, PartialEq, Eq)]
//! enum Error {
//!     Utf8(Utf8Error),
//!     ParseInt(ParseIntError),
//! }
//!
//! impl From<Utf8Error> for Error {
//!     fn from(value: Utf8Error) -> Error {
//!         Error::Utf8(value)
//!     }
//! }
//!
//! impl From<ParseIntError> for Error {
//!     fn from(value: ParseIntError) -> Error {
//!         Error::ParseInt(value)
//!     }
//! }
//!
//! fn find_number(number: i32, list: &str) -> SearchResult<Range, Error> {
//!     subslice::binary_search_by_key(&number, list.as_bytes(), |search_slice| {
//!         let located = search_slice.subslice_range_from_midpoint_to_delimiters(|&b| b == b',');
//!         let n_bytes = &search_slice[located.subslice_range];
//!         let n_str = str::from_utf8(n_bytes)?;
//!         let n: i32 = n_str.parse()?;
//!         Ok(Some(LocatedItem {
//!             value: n,
//!             value_range: located.subslice_range,
//!             consumed_range: located.consumed_range,
//!         }))
//!     })
//! }
//!
//! let numbers = "9,87,654,3210";
//!
//! assert_eq!(find_number(1, &numbers), Ok(Err(0)));
//! assert_eq!(find_number(9, &numbers), Ok(Ok((0..1).into())));
//! assert_matches!(find_number(48, &numbers), Ok(Err(1..=2)));
//! assert_eq!(find_number(87, &numbers), Ok(Ok((2..4).into())));
//! assert_matches!(find_number(370, &numbers), Ok(Err(4..=5)));
//! assert_eq!(find_number(654, &numbers), Ok(Ok((5..8).into())));
//! assert_matches!(find_number(1932, &numbers), Ok(Err(8..=9)));
//! assert_eq!(find_number(3210, &numbers), Ok(Ok((9..13).into())));
//! assert_matches!(find_number(4000, &numbers), Ok(Err(13..=13)));
//! ```
//!
//! ## Lines in a text file<a id="lines-in-a-text-file"></a>
//!
//! Using [`line::buffered::binary_search`] to read the file on demand:
//!
//! ```
//! use std::fs::File;
//! use std::num::NonZero;
//! use std::path::Path;
//!
//! use binary_search_collection::{Range, line};
//!
//! fn find_line_in_file<P: AsRef<Path>>(line: &str, path: P) -> Option<Range> {
//!     let file = File::open(path).unwrap();
//!     let buffer_len = NonZero::try_from(8 * 1024).unwrap();
//!     line::buffered::binary_search(line, file, buffer_len)
//!         .unwrap()
//!         .ok()
//! }
//! ```
//!
//! ## Lines from a text file loaded in memory<a id="lines-from-a-text-file-in-memory"></a>
//!
//! Using [`line::binary_search`] with the content of a text file fully loaded into the memory:
//!
//! ```
//! use std::fs::read;
//! use std::path::Path;
//!
//! use binary_search_collection::{Range, line};
//!
//! fn find_line_in_file<P: AsRef<Path>>(line: &str, path: P) -> Option<Range> {
//!     let bytes = read(path).unwrap();
//!     line::binary_search(line, &bytes).ok()
//! }
//! ```
//!
//! ## Numeric lines in a text file<a id="numeric-lines-in-a-text-file"></a>
//!
//! Using [`line::buffered::binary_search_by_key`]:
//!
//! ```
//! use std::fs::File;
//! use std::io::Error as IoError;
//! use std::num::{NonZero, ParseIntError};
//! use std::path::Path;
//! use std::str::Utf8Error;
//!
//! use binary_search_collection::{Range, line};
//!
//! enum Error {
//!     Io(IoError),
//!     Utf8(Utf8Error),
//!     ParseInt(ParseIntError),
//! }
//!
//! impl From<IoError> for Error {
//!     fn from(value: IoError) -> Error {
//!         Error::Io(value)
//!     }
//! }
//!
//! impl From<Utf8Error> for Error {
//!     fn from(value: Utf8Error) -> Error {
//!         Error::Utf8(value)
//!     }
//! }
//!
//! impl From<ParseIntError> for Error {
//!     fn from(value: ParseIntError) -> Error {
//!         Error::ParseInt(value)
//!     }
//! }
//!
//! fn find_number_line_in_file<P: AsRef<Path>>(
//!     number: i32,
//!     path: P,
//! ) -> Result<Option<Range>, Error> {
//!     let file = File::open(path).unwrap();
//!     let buffer_len = NonZero::try_from(8 * 1024).unwrap();
//!     line::buffered::binary_search_by_key(&number, file, buffer_len, |line_bytes| {
//!         let n_bytes: Vec<_> = line_bytes.collect::<Result<_, _>>()?;
//!         let n_str = str::from_utf8(&n_bytes)?;
//!         let n = n_str.parse()?;
//!         Ok(n)
//!     })
//!     .map(Result::ok)
//! }
//! ```
//!
//! ## Regular binary search<a id="regular-binary-search"></a>
//!
//! Just like [`slice::binary_search`].
//!
//! Using [`subslice::binary_search`]:
//!
//! ```
//! use std::convert::Infallible;
//!
//! use binary_search_collection::Range;
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use binary_search_collection::subslice::{self, LocatedSubslice};
//!
//! fn regular_binary_search<T: Ord>(value: &T, list: &[T]) -> Result<usize, usize> {
//!     let value_slice = core::slice::from_ref(value);
//!     let Ok(result) = subslice::binary_search::<_, Infallible>(value_slice, list, |slice| {
//!         let value_range = Range::from_start_and_len(slice.range().midpoint(), 1);
//!         Ok(Some(LocatedSubslice {
//!             subslice_range: value_range,
//!             consumed_range: value_range,
//!         }))
//!     });
//!
//!     result.map(|range| range.start)
//! }
//!
//! assert_eq!(regular_binary_search(&5, &[10, 20, 30]), Err(0));
//! assert_eq!(regular_binary_search(&10, &[10, 20, 30]), Ok(0));
//! assert_eq!(regular_binary_search(&15, &[10, 20, 30]), Err(1));
//! assert_eq!(regular_binary_search(&20, &[10, 20, 30]), Ok(1));
//! assert_eq!(regular_binary_search(&25, &[10, 20, 30]), Err(2));
//! assert_eq!(regular_binary_search(&30, &[10, 20, 30]), Ok(2));
//! assert_eq!(regular_binary_search(&35, &[10, 20, 30]), Err(3));
//! ```
use crate::ext::Offset;

pub mod ext;
pub mod line;
pub mod sparse;
pub mod subslice;

pub type Range = core::range::Range<usize>;

impl Offset for Range {
    fn offset(mut self, amount: usize) -> Self {
        self.start = self.start.offset(amount);
        self.end = self.end.offset(amount);
        self
    }
}

/// The return type for functions where the callback may fail.
///
/// The value can be:
///
/// - `Ok(Ok(x))`: The target value was found.
/// - `Ok(Err(i))`: The target value was _not_ found. The value could be inserted at the index `i`
///   in the source while maintaining the sort order.
/// - `Err(e)`: The callback closure failed with the error `e`.
pub type SearchResult<T, E> = Result<Result<T, usize>, E>;

/// A value and its location in the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocatedItem<T> {
    /// The value that is compared during the binary search.
    ///
    /// It may be a subslice reference of the source, or may be a new value derived from the
    /// source.
    pub value: T,

    /// The range in the source that delimits the located value.
    ///
    /// It must be a subrange of `consumed_range`.
    pub value_range: Range,

    /// The range in the source slice that was "consumed" in the search for the value.
    ///
    /// It may include delimiters that were used to determine the `value_range`, and is used to
    /// delimit the resuming search when the located value does not correspond to the target.
    pub consumed_range: Range,
}

impl<T> Offset for LocatedItem<T> {
    fn offset(self, amount: usize) -> Self {
        LocatedItem {
            value: self.value,
            value_range: self.value_range.offset(amount),
            consumed_range: self.consumed_range.offset(amount),
        }
    }
}

/// Indicates how the search must proceed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparison {
    /// The search must proceed before the specified position.
    Before(usize),
    /// The value was found in the specified range.
    Found(Range),
    /// The search must proceed after the specified position.
    After(usize),
}

impl Offset for Comparison {
    fn offset(mut self, amount: usize) -> Self {
        match &mut self {
            Comparison::Found(range) => *range = range.offset(amount),
            Comparison::Before(x) | Comparison::After(x) => *x = x.offset(amount),
        }
        self
    }
}
