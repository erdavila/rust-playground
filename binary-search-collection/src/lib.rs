#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Provides binary search functions for ranges where distinct positions may not correspond to distinct values.
//!
//! This includes cases where some positions are ignored, or where values can span across multiple
//! positions. Also, the binary search may be run in an unspecified source, e.b. by loading data
//! on-demand.
//!
//! It is important to define the following value types:
//!
//! - _elements_: The low-level values. Indexes and ranges refer to them.
//! - _items_: The values that are compared during the search. They are somehow derived from the _elements_.
//!
//! # Cases and Examples
//!
//! ## Values with gaps<a id="values-with-gaps"></a>
//!
//! A list of values where some of them must be ignored.
//!
//! Using [`sparse::binary_search`]:
//!
//! ```
//! use binary_search_collection::{sparse, LocatedItem, Range};
//! use binary_search_collection::ext::RangeExt as _;
//! use std::assert_matches;
//! use std::convert::Infallible;
//!
//! fn binary_search_with_gaps<T, V: Ord>(value: &V, list: &[T], mut extract: impl FnMut(&T) -> Option<V>) -> Result<usize, usize> {
//!     let Ok(result) = sparse::binary_search::<_, _, Infallible>(value, list.len(), |range| {
//!         let li_opt = range
//!             .iter_from_midpoint()
//!             .find_map(|i| {
//!                 extract(&list[i]).map(|v| {
//!                     let value_range = Range::from_start_and_len(i, 1);
//!                     LocatedItem {
//!                         value: v,
//!                         value_range,
//!                         consumed_range: value_range,
//!                     }
//!                 })
//!             });
//!         Ok(li_opt)
//!     });
//!
//!     result.map(|range| range.start)
//! }
//!
//! let list = [Ok(10), Ok(20), Err("ignored"), Err("not found"), Ok(30), Err("failed")];
//!
//! assert_eq!(     binary_search_with_gaps(&5, &list, |result| result.ok()), Err(0));
//! assert_eq!(     binary_search_with_gaps(&10, &list, |result| result.ok()), Ok(0));
//! assert_eq!(     binary_search_with_gaps(&15, &list, |result| result.ok()), Err(1));
//! assert_eq!(     binary_search_with_gaps(&20, &list, |result| result.ok()), Ok(1));
//! assert_matches!(binary_search_with_gaps(&25, &list, |result| result.ok()), Err(2..=4));
//! assert_eq!(     binary_search_with_gaps(&30, &list, |result| result.ok()), Ok(4));
//! assert_matches!(binary_search_with_gaps(&35, &list, |result| result.ok()), Err(5..=6));
//! ```
//!
//! ## Subslices with gaps<a id="subslices-with-gaps"></a>
//!
//! A list of _items_ that span across _elements_, separated by _elements_ that may occupy multiple
//! consecutive positions.
//!
//! Using [`subslice::binary_search`]:
//!
//! ```
//! use binary_search_collection::{subslice, Range};
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use std::assert_matches;
//! use std::convert::Infallible;
//!
//! fn find_subslice_with_gaps<T: Ord>(
//!     subslice: &[T],
//!     source: &[T],
//!     gap: &T,
//! ) -> Result<Range, usize> {
//!     let Ok(result) = subslice::binary_search::<_, Infallible>(subslice, source, |slice| {
//!         let located_subslice_opt = slice
//!             .locate_from_midpoint(|x| x != gap)
//!             .map(|non_gap_index| {
//!                 let non_gap_range = Range::from_start_and_len(non_gap_index, 1);
//!                 slice.extend_subslice_range_to_delimiters(non_gap_range, |x| x == gap)
//!             });
//!         Ok(located_subslice_opt)
//!     });
//!
//!     result
//! }
//!
//! let elements = ['a', 'a', '-', '-', '-', 'b', 'b', '-', 'c', 'c', '-', '-'];
//!
//! assert_eq!(     find_subslice_with_gaps(&['a'], &elements, &'-'), Err(0));
//! assert_eq!(     find_subslice_with_gaps(&['a', 'a'], &elements, &'-'), Ok((0..2).into()));
//! assert_matches!(find_subslice_with_gaps(&['b'], &elements, &'-'), Err(2..=5));
//! assert_eq!(     find_subslice_with_gaps(&['b', 'b'], &elements, &'-'), Ok((5..7).into()));
//! assert_matches!(find_subslice_with_gaps(&['c'], &elements, &'-'), Err(7..=8));
//! assert_eq!(     find_subslice_with_gaps(&['c', 'c'], &elements, &'-'), Ok((8..10).into()));
//! assert_matches!(find_subslice_with_gaps(&['d'], &elements, &'-'), Err(10..=12));
//! ```
//!
//! ## Subslices with delimiters<a id="subslices-with-delimiters"></a>
//!
//! A list of _items_ that span across _elements_, using a delimiter.
//!
//! Using [`subslice::binary_search`]:
//!
//! ```
//! use binary_search_collection::{subslice, Range};
//! use binary_search_collection::ext::SliceExt as _;
//! use std::assert_matches;
//! use std::convert::Infallible;
//!
//! fn find_delimited(sequence: &[char], chars: &[char], delimiter: char) -> Result<Range, usize> {
//!     let Ok(result) = subslice::binary_search::<_, Infallible>(sequence, chars, |slice| {
//!         let located_subslice = slice.subslice_range_from_midpoint_to_delimiters(|&c| c == delimiter);
//!         Ok(Some(located_subslice))
//!     });
//!
//!     result
//! }
//!
//! let chars = ['b', 'b', '-', 'd', 'd', '-', 'f', 'f'];
//!
//! assert_eq!(     find_delimited(&['a', 'a'], &chars, '-'), Err(0));
//! assert_eq!(     find_delimited(&['b', 'b'], &chars, '-'), Ok((0..2).into()));
//! assert_matches!(find_delimited(&['c', 'c'], &chars, '-'), Err(2..=3));
//! assert_eq!(     find_delimited(&['d', 'd'], &chars, '-'), Ok((3..5).into()));
//! assert_matches!(find_delimited(&['e', 'e'], &chars, '-'), Err(5..=6));
//! assert_eq!(     find_delimited(&['f', 'f'], &chars, '-'), Ok((6..8).into()));
//! assert_eq!(     find_delimited(&['g', 'g'], &chars, '-'), Err(8));
//! ```
//!
//! ## Subslices with delimiters With a custom comparison<a id="subslices-with-delimiters-with-a-custom-comparison"></a>
//!
//! A custom comparison can be used with [`sparse::binary_search`].
//!
//! In the example below, slices are compared in reverse order:
//!
//! ```
//! use binary_search_collection::{sparse, LocatedItem, Range};
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use std::assert_matches;
//! use std::cmp::Ordering;
//! use std::convert::Infallible;
//!
//! fn reverse_find_delimited(sequence: &[char], chars: &[char], delimiter: char) -> Result<Range, usize> {
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
//!     let Ok(result) = sparse::binary_search::<_, _, Infallible>(&Reverse(sequence), chars.len(), |range| {
//!         let located_subslice = chars
//!             .in_range(range, |slice| {
//!                 slice.subslice_range_from_midpoint_to_delimiters(|&c| c == delimiter)
//!             });
//!
//!         let value_range = located_subslice.subslice_range;
//!         let value = Reverse(&chars[value_range]);
//!         let consumed_range = located_subslice.consumed_range;
//!
//!         Ok(Some(LocatedItem {
//!             value,
//!             value_range,
//!             consumed_range,
//!         }))
//!     });
//!
//!     result
//! }
//!
//! let chars = ['f', 'f', '-', 'd', 'd', '-', 'b', 'b'];
//!
//! assert_eq!(     reverse_find_delimited(&['g', 'g'], &chars, '-'), Err(0));
//! assert_eq!(     reverse_find_delimited(&['f', 'f'], &chars, '-'), Ok((0..2).into()));
//! assert_matches!(reverse_find_delimited(&['e', 'e'], &chars, '-'), Err(2..=3));
//! assert_eq!(     reverse_find_delimited(&['d', 'd'], &chars, '-'), Ok((3..5).into()));
//! assert_matches!(reverse_find_delimited(&['c', 'c'], &chars, '-'), Err(5..=6));
//! assert_eq!(     reverse_find_delimited(&['b', 'b'], &chars, '-'), Ok((6..8).into()));
//! assert_eq!(     reverse_find_delimited(&['a', 'a'], &chars, '-'), Err(8));
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
//! use binary_search_collection::Range;
//! use binary_search_collection::line;
//!
//! fn locate_line_in_file<P: AsRef<Path>>(line: &str, path: P) -> Option<Range> {
//!     let file = File::open(path).unwrap();
//!     let buffer_len = NonZero::try_from(8 * 1024).unwrap();
//!     line::buffered::binary_search(line, file, buffer_len).unwrap().ok()
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
//! use binary_search_collection::Range;
//! use binary_search_collection::line;
//!
//! fn locate_line_in_file<P: AsRef<Path>>(line: &str, path: P) -> Option<Range> {
//!     let bytes = read(path).unwrap();
//!     line::binary_search(line, &bytes).ok()
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
//! use binary_search_collection::Range;
//! use binary_search_collection::subslice::{self, LocatedSubslice};
//! use binary_search_collection::ext::{RangeExt as _, SliceExt as _};
//! use std::convert::Infallible;
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
/// - `Ok(Ok(x))`: The _item_ was found.
/// - `Ok(Err(i))`: The _item_ was NOT found. It could be inserted at the index `i` in the source
///   _elements_ while maintaining the sort order.
/// - `Err(e)`: The callback closure failed with the error `e`.
pub type SearchResult<T, E> = Result<Result<T, usize>, E>;

/// The value and its location from an `elements` range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocatedItem<T> {
    /// The _item_ that is compared during the binary search.
    ///
    /// It may be a subslice reference of the source _elements_, or may be a new value derived from
    /// the source _elements_.
    pub value: T,

    /// The range of the _elements_ that were directly used to derive the `value`.
    ///
    /// It must be a subslice of `consumed_range`.
    pub value_range: Range,

    /// The range of _elements_ that were "consumed" in the search for the subslice.
    ///
    /// It may include delimiters that were used to locate the _elements_ that derived the `value`.
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
