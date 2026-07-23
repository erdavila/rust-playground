#![no_std]

//! Provides binary search functions for ranges where distinct positions may not correspond to distinct values.
//!
//! This includes cases where some positions are ignored, or where values can span across multiple positions.
//!
//! It is important to define the following value types:
//!
//! - _elements_ - The low-level values. Indexes and ranges refer to them.
//! - _items_ - The values that are compared during the search. They are somehow derived from the _elements_.
//!
//! The variants include:
//!
//! - searching in slices, or in an unspecified source, where the _elements_ are fetched on-demand;
//! - searching for subslices, or for values of an arbitrary type;
//! - searching specifically for lines as from a text file.

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::ops::{Bound, RangeBounds};

pub type Range = core::range::Range<usize>;

impl Offset for Range {
    fn offset(mut self, amount: usize) -> Self {
        self.start = self.start.offset(amount);
        self.end = self.end.offset(amount);
        self
    }
}

pub type SearchResult<T> = Result<T, usize>;

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
/// ```
/// use binary_search_collection::{sparse_binary_search, LocatedItem, Range, SearchResult, RangeExt as _, SliceExt as _};
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
///     sparse_binary_search(&Reverse(sequence), chars.len(), |range| {
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
pub fn sparse_binary_search<T, Q>(
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
/// use binary_search_collection::{subslice_binary_search, Range, SearchResult, SliceExt as _};
/// use std::assert_matches;
///
/// fn find_delimited(sequence: &[char], chars: &[char], delimiter: char) -> SearchResult<Range> {
///     subslice_binary_search(sequence, chars, |slice| {
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
/// use binary_search_collection::{subslice_binary_search, Range, SearchResult, RangeExt as _, SliceExt as _};
/// use std::assert_matches;
///
/// fn find_subslice_with_gaps<T: Ord>(
///     subslice: &[T],
///     source: &[T],
///     gap: &T,
/// ) -> SearchResult<Range> {
///     subslice_binary_search(subslice, source, |slice| {
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
#[expect(clippy::missing_errors_doc)]
pub fn subslice_binary_search<'a, T: Ord>(
    target_subslice: &[T],
    source: &'a [T],
    mut locate_subslice_in: impl FnMut(&'a [T]) -> Option<LocatedSubslice>,
) -> SearchResult<Range> {
    sparse_binary_search(target_subslice, source.len(), |search_range| {
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

const CR: u8 = b'\r';
const LF: u8 = b'\n';

/// Executes a binary search of a text line.
///
/// Lines are delimited by line breaks, which are a line feed (`\n`) and may include a preceeding
/// carriage return (`\r`) when present. The last line may not have a line break.
///
/// When the `target_line` is found, its byte range without the line break is returned. If the
/// `target_line` is not found then [`Err`] is returned, containing the index where the
/// `target_line` could be inserted while maintaining sorted order.
///
/// # Example
///
/// ```
/// use std::fs::read;
/// use std::path::Path;
/// use std::range::Range;
/// use binary_search_collection::line_binary_search;
///
/// fn locate_line_in_file<P: AsRef<Path>>(line: &str, path: P) -> Option<Range<usize>> {
///     let bytes = read(path).unwrap();
///     line_binary_search(line, &bytes).ok()
/// }
/// ```
#[expect(clippy::missing_errors_doc)]
pub fn line_binary_search(target_line: impl AsRef<[u8]>, bytes: &[u8]) -> SearchResult<Range> {
    subslice_binary_search(target_line.as_ref(), bytes, |search_slice| {
        let ls = search_slice.subslice_range_from_midpoint_to_delimiters(|&b| b == LF);

        let line_start = ls.subslice_range.start;
        let range_with_break = (line_start..ls.consumed_range.end).into();

        let line_with_break = &search_slice[range_with_break];
        let line_without_break = strip_line_break(line_with_break);

        let range_without_break = Range::from_start_and_len(line_start, line_without_break.len());

        Some(LocatedSubslice {
            subslice_range: range_without_break,
            consumed_range: range_with_break,
        })
    })
}

#[must_use]
fn strip_line_break(line: &[u8]) -> &[u8] {
    fn strip_last(b: u8, s: &[u8]) -> Option<&[u8]> {
        (s.last() == Some(&b)).then(|| &s[..s.len() - 1])
    }

    let Some(line) = strip_last(LF, line) else {
        return line;
    };
    let Some(line) = strip_last(CR, line) else {
        return line;
    };

    line
}

/// Extension methods for [`Range<usize>`].
pub trait RangeExt: Copy {
    fn from_start_and_len(start: usize, len: usize) -> Self;

    fn is_subrange_of(self, other: Self) -> bool;

    fn midpoint(self) -> usize;

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

    fn midpoint(self) -> usize {
        self.start.midpoint(self.end)
    }

    fn iter_from_midpoint(self) -> IterFromMidpoint {
        IterFromMidpoint {
            counter: 0,
            limit: self.end.saturating_sub(self.start),
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

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    mod line_binary_search {
        use alloc::collections::BTreeMap;
        use alloc::vec::Vec;

        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum LineBreak {
            Lf,
            CrLf,
        }

        impl LineBreak {
            #[must_use]
            fn bytes(self) -> &'static [u8] {
                match self {
                    LineBreak::Lf => &[LF],
                    LineBreak::CrLf => &[CR, LF],
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct LineLocation {
            range: Range,
            line_break_len: usize,
        }

        fn make_bytes<'a, L: AsRef<[u8]> + Ord + ?Sized>(
            lines: impl IntoIterator<Item = &'a L>,
            line_break: LineBreak,
            final_line_break: bool,
        ) -> (Vec<u8>, BTreeMap<&'a L, LineLocation>) {
            let mut bytes = Vec::new();
            let mut ranges = BTreeMap::new();

            let mut lines = lines.into_iter().peekable();

            while let Some(line) = lines.next() {
                let start = bytes.len();
                bytes.extend_from_slice(line.as_ref());
                let end = bytes.len();

                let line_break_len = if final_line_break || lines.peek().is_some() {
                    let line_break_bytes = line_break.bytes();
                    bytes.extend(line_break_bytes);
                    line_break_bytes.len()
                } else {
                    0
                };

                ranges.insert(
                    line,
                    LineLocation {
                        range: (start..end).into(),
                        line_break_len,
                    },
                );
            }

            (bytes, ranges)
        }

        macro_rules! assert_line_found {
            ($result:expr, $line_location:expr) => {{
                let range = $result.unwrap();
                assert_eq!(range, $line_location.range);
            }};
        }

        macro_rules! assert_line_not_found {
            ($result:expr, before: $line_location:expr) => {
                assert_line_not_found!(@ $result, $line_location.range.start);
            };
            ($result:expr, after: $line_location:expr) => {
                let line_location = $line_location;
                assert_line_not_found!(@ $result, line_location.range.end + line_location.line_break_len);
            };
            (@ $result:expr, $next_line_index:expr) => {
                assert_eq!(
                    $result,
                    Err($next_line_index)
                );
            };
        }

        #[test]
        fn empty() {
            assert_eq!(line_binary_search(b"a", &[]), Err(0));
        }

        #[test]
        fn lf() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::Lf, true);
            assert_eq!(bytes.len(), 15);

            assert_line_found!(line_binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(line_binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(line_binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(line_binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(line_binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(line_binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(line_binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(line_binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(line_binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(line_binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(line_binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn crlf() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::CrLf, true);
            assert_eq!(bytes.len(), 20);

            assert_line_found!(line_binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(line_binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(line_binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(line_binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(line_binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(line_binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(line_binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(line_binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(line_binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(line_binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(line_binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn lf_no_final_line_break() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::Lf, false);
            assert_eq!(bytes.len(), 14);

            assert_line_found!(line_binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(line_binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(line_binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(line_binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(line_binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(line_binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(line_binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(line_binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(line_binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(line_binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(line_binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn crlf_no_final_line_break() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::CrLf, false);
            assert_eq!(bytes.len(), 18);

            assert_line_found!(line_binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(line_binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(line_binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(line_binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(line_binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(line_binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(line_binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(line_binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(line_binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(line_binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(line_binary_search("f", &bytes), after: locs["ee"]);
        }
    }

    mod strip_line_break {
        use super::*;

        const A: u8 = b'a';
        const B: u8 = b'b';

        #[test]
        fn empty() {
            assert_eq!(strip_line_break(&[]), []);
        }

        #[test]
        fn lf_only() {
            assert_eq!(strip_line_break(&[LF]), []);
        }

        #[test]
        fn crlf_only() {
            assert_eq!(strip_line_break(&[CR, LF]), []);
        }

        #[test]
        fn no_lf() {
            assert_eq!(strip_line_break(&[A, B]), [A, B]);
        }

        #[test]
        fn lf() {
            assert_eq!(strip_line_break(&[A, B, LF]), [A, B]);
        }

        #[test]
        fn crlf() {
            assert_eq!(strip_line_break(&[A, B, CR, LF]), [A, B]);
        }

        #[test]
        fn cr_without_lf() {
            assert_eq!(strip_line_break(&[A, B, CR]), [A, B, CR]);
        }
    }

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
