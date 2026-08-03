//! Binary search of lines as loaded from a text file.
//!
//! Lines are delimited by line breaks, which are a line feed (`\n`) and may include a preceeding
//! carriage return (`\r`) when present. The last line may not have a line break.

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::Debug;
#[cfg(feature = "std")]
use std::collections::vec_deque::VecDeque;

use crate::ext::{ByteSliceExt, Offset, RangeExt as _, SliceExt as _};
use crate::{Comparison, Range, SearchResult, subslice};

#[cfg(feature = "std")]
pub mod buffered;

pub(crate) const CR: u8 = b'\r';
pub(crate) const LF: u8 = b'\n';

/// Executes a binary search of a text line.
///
/// When the `target_line` is found, its byte range without the line break is returned. If the
/// `target_line` is not found then [`Err`] is returned, containing the index where the
/// `target_line` could be inserted while maintaining sorted order.
///
/// To execute a binary search by on-demand loading the content of a file, use
/// [`line::buffered::binary_search`].
///
/// This function is also available as an [extension method for slices](crate::ext::ByteSliceExt::line_binary_search).
///
/// # Example
///
/// - [Lines from a text file load in memory](crate#lines-from-a-text-file-in-memory)
#[expect(clippy::missing_errors_doc)]
pub fn binary_search(target_line: impl AsRef<[u8]>, bytes: &[u8]) -> Result<Range, usize> {
    binary_search::implementation(target_line, bytes)
}

// Implementation using `line::binary_search_by_key`.
#[cfg(not(feature = "alternative-line-binary_search"))]
mod binary_search {
    use core::convert::Infallible;

    use crate::{Range, line};

    pub(super) fn implementation(
        target_line: impl AsRef<[u8]>,
        bytes: &[u8],
    ) -> Result<Range, usize> {
        let Ok(result) = line::binary_search_by_key::<_, _, Infallible>(
            target_line.as_ref(),
            bytes,
            |line_bytes| {
                let slice = line_bytes.remainder();
                Ok(slice)
            },
        );

        result
    }
}

// Implementation using `subslice::binary_search_by`.
#[cfg(feature = "alternative-line-binary_search")]
mod binary_search {
    use core::cmp::Ordering;
    use core::convert::Infallible;

    use crate::line::LineBytes;
    use crate::{Comparison, Range, subslice};

    pub(super) fn implementation(
        target_line: impl AsRef<[u8]>,
        bytes: &[u8],
    ) -> Result<Range, usize> {
        let target_line = target_line.as_ref();
        let Ok(result) = subslice::binary_search_by::<_, Infallible>(bytes, |search_slice| {
            let mut line_bytes = LineBytes::from_midpoint_of(search_slice);
            let line_start = line_bytes.next_index;
            let line_prefix = line_bytes.next_slice(target_line.len());

            let cmp = match line_prefix.cmp(target_line) {
                Ordering::Less => {
                    let line_end = line_bytes.skip_to_end();
                    let next_line_start = line_end.line_break_end_position();
                    Comparison::After(next_line_start)
                }
                Ordering::Equal => {
                    // The located line has `line` as a prefix.
                    if line_bytes.next().is_some() {
                        // The located line has more bytes than `line`.
                        Comparison::Before(line_start)
                    } else {
                        // The located line and the `line` are equal.
                        let line_end = line_bytes.skip_to_end();
                        let located_line_end = line_end.position;
                        Comparison::Found((line_start..located_line_end).into())
                    }
                }
                Ordering::Greater => Comparison::Before(line_start),
            };

            Ok(Some(cmp))
        });
        result
    }
}

/// Executes a binary search of a text line with a comparison key extraction.
///
/// The `extract` closure must read the bytes in its [`LineBytes`] parameter and extract a value of
/// type `V` to be compared against the `target_value`.
///
/// When the target line is found, its byte range without the line break is returned. If the target
/// line is not found then [`Err`] is returned, containing the index where the target line could be
/// inserted while maintaining sorted order.
///
/// This function is also available as an [extension method for slices](crate::ext::ByteSliceExt::line_binary_search_by_key).
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by_key<'a, T, Q, E>(
    target_value: &Q,
    bytes: &'a [u8],
    extract: impl FnMut(&mut LineBytes<'a>) -> Result<T, E>,
) -> SearchResult<Range, E>
where
    Q: Ord + ?Sized,
    T: Borrow<Q>,
{
    binary_search_by_key::implementation(target_value, bytes, extract)
}

// Implementation using `line::binary_search_by`.
#[cfg(not(feature = "alternative-line-binary_search_by_key"))]
mod binary_search_by_key {
    use core::borrow::Borrow;

    use crate::line::{self, LineBytes};
    use crate::{Range, SearchResult};

    pub(super) fn implementation<'a, T, Q, E>(
        target_value: &Q,
        bytes: &'a [u8],
        mut extract: impl FnMut(&mut LineBytes<'a>) -> Result<T, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        line::binary_search_by(bytes, |line_bytes| {
            extract(line_bytes).map(|value| value.borrow().cmp(target_value))
        })
    }
}

// Implementation using `subslice::binary_search_by_key`.
#[cfg(feature = "alternative-line-binary_search_by_key")]
mod binary_search_by_key {
    use core::borrow::Borrow;

    use crate::line::LineBytes;
    use crate::{LocatedItem, Range, SearchResult, subslice};

    pub(super) fn implementation<'a, T, Q, E>(
        target_value: &Q,
        bytes: &'a [u8],
        mut extract: impl FnMut(&mut LineBytes<'a>) -> Result<T, E>,
    ) -> SearchResult<Range, E>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        subslice::binary_search_by_key::<_, T, _, _>(target_value, bytes, |search_slice| {
            let mut line_bytes = LineBytes::from_midpoint_of(search_slice);
            let start = line_bytes.next_index;

            let value = extract(&mut line_bytes)?;

            let line_end = line_bytes.skip_to_end();
            Ok(Some(LocatedItem {
                value,
                value_range: (start..line_end.position).into(),
                consumed_range: (start..line_end.line_break_end_position()).into(),
            }))
        })
    }
}

/// Executes a binary search of a text line with a custom comparison.
///
/// The `compare` closure must read the bytes in its [`LineBytes`] parameter and compare it with the
/// target line, returning the proper [`Ordering`] value.
///
/// When the target line is found, its byte range without the line break is returned. If the target
/// line is not found then [`Err`] is returned, containing the index where the target line could be
/// inserted while maintaining sorted order.
///
/// This function is also available as an [extension method for slices](crate::ext::ByteSliceExt::line_binary_search_by).
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by<'a, E>(
    bytes: &'a [u8],
    mut compare: impl FnMut(&mut LineBytes<'a>) -> Result<Ordering, E>,
) -> SearchResult<Range, E> {
    subslice::binary_search_by(bytes, |search_slice| {
        let mut line_bytes = LineBytes::from_midpoint_of(search_slice);
        let start = line_bytes.next_index;

        let cmp = match compare(&mut line_bytes)? {
            Ordering::Less => {
                let line_end = line_bytes.skip_to_end();
                let next_line_start = line_end.line_break_end_position();
                Comparison::After(next_line_start)
            }
            Ordering::Equal => {
                let line_end = line_bytes.skip_to_end();
                let end = line_end.position;
                Comparison::Found((start..end).into())
            }
            Ordering::Greater => Comparison::Before(start),
        };

        Ok(Some(cmp))
    })
}

/// An iterator on the bytes of a line.
pub struct LineBytes<'a> {
    bytes: &'a [u8],
    next_index: usize,
}

impl<'a> LineBytes<'a> {
    /// Creates a new instance for the line that includes the midpoint of `slice`.
    pub fn from_midpoint_of(slice: &'a [u8]) -> Self {
        let midpoint = slice.range().midpoint();
        let start = slice.in_range(..midpoint, ByteSliceExt::locate_line_start_from_end_or_zero);
        LineBytes {
            bytes: slice,
            next_index: start,
        }
    }

    /// Returns a slice of up to `len` bytes.
    pub fn next_slice(&mut self, len: usize) -> &'a [u8] {
        self.slice(|this| {
            // Advances up to `len` bytes.
            for _ in 0..len {
                if this.next().is_none() {
                    break;
                }
            }
        })
    }

    /// Returns a slice with the remaining bytes.
    pub fn remainder(&mut self) -> &'a [u8] {
        self.slice(|this| {
            this.skip_to_end();
        })
    }

    fn slice(&mut self, f: impl FnOnce(&mut Self)) -> &'a [u8] {
        let start = self.next_index;
        f(self);
        let end = self.next_index;
        &self.bytes[start..end]
    }

    /// Skips to the end of the line.
    pub fn skip_to_end(&mut self) -> LineEnd {
        let line_end = self.bytes.in_range(
            self.next_index..,
            ByteSliceExt::locate_line_end_from_start_or_len,
        );

        self.next_index = line_end.position;
        line_end
    }
}

impl Iterator for LineBytes<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.bytes.get(self.next_index).and_then(|&b| match b {
            LF => None,
            CR => {
                if self.bytes.get(self.next_index + 1) == Some(&LF) {
                    None
                } else {
                    Some(b)
                }
            }
            _ => Some(b),
        });

        if output.is_some() {
            self.next_index += 1;
        }
        output
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineEnd {
    pub position: usize,
    pub line_break_len: usize,
}

impl LineEnd {
    #[must_use]
    pub fn line_break_end_position(self) -> usize {
        self.position + self.line_break_len
    }
}

impl Offset for LineEnd {
    fn offset(mut self, amount: usize) -> Self {
        self.position = self.position.offset(amount);
        self
    }
}

#[allow(dead_code)]
trait DebugStr {
    type Output<'a>: AsRef<str> + Debug
    where
        Self: 'a;
    fn debug_str(&self) -> Self::Output<'_>;
}

impl DebugStr for [u8] {
    type Output<'a>
        = &'a str
    where
        Self: 'a;

    fn debug_str(&self) -> Self::Output<'_> {
        unsafe { str::from_utf8_unchecked(self) }
    }
}

#[cfg(feature = "std")]
impl DebugStr for VecDeque<u8> {
    type Output<'a>
        = String
    where
        Self: 'a;

    fn debug_str(&self) -> Self::Output<'_> {
        unsafe { String::from_utf8_unchecked(self.iter().copied().collect()) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) enum LineBreak {
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
    pub(crate) struct LineLocation {
        pub(crate) range: Range,
        pub(crate) line_break_len: usize,
    }

    pub(crate) fn make_bytes<'a, L: AsRef<[u8]> + Ord + ?Sized>(
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

    mod make_bytes {
        use super::*;

        #[test]
        fn test() {
            let lines = ["abc", "xyz"];

            macro_rules! case {
                ($line_break:expr, $final_line_break:expr, $expected_bytes:expr $(; $line:expr => $expected_range:expr, $expected_with_line_break:expr)*) => {
                    let line_break = $line_break;
                    let (bytes, locs) = make_bytes(lines, line_break, $final_line_break);
                    assert_eq!(bytes, $expected_bytes);
                    assert_eq!(locs, BTreeMap::from([
                        $(
                            (
                                $line,
                                LineLocation {
                                    range: $expected_range.into(),
                                    line_break_len: if $expected_with_line_break { line_break.bytes().len() } else { 0 },
                                },
                            ),
                        )*
                    ]));
                };
            }

            case!(LineBreak::Lf, true, b"abc\nxyz\n"; "abc" => 0..3, true; "xyz" => 4..7, true);
            case!(LineBreak::Lf, false, b"abc\nxyz"; "abc" => 0..3, true; "xyz" => 4..7, false);
            case!(LineBreak::CrLf, true, b"abc\r\nxyz\r\n"; "abc" => 0..3, true; "xyz" => 5..8, true);
            case!(LineBreak::CrLf, false, b"abc\r\nxyz"; "abc" => 0..3, true; "xyz" => 5..8, false);
        }
    }

    macro_rules! assert_line_found {
        ($result:expr, $line_location:expr $(, $($arg:tt)*)?) => {{
            let result = $result;
            let Ok(range) = result else {
                $crate::line::tests::assert_line_found!(@panic "Expected Result::Ok instead of {:?}", result $(, $($arg)*)?);
            };
            assert_eq!(range, $line_location.range $(, $($arg)*)?);
        }};
        (@panic $msg:tt, $result:expr $(,)?) => {
            panic!($msg, $result);
        };
        (@panic $msg:tt, $result:expr, $($arg:tt)+) => {
            panic!(concat!($msg, ": {}"), $result, format_args!($($arg)+) );
        };
    }
    pub(crate) use assert_line_found;

    macro_rules! assert_line_not_found {
        ($result:expr, before: $line_location:expr $(, $($arg:tt)*)?) => {
            $crate::line::tests::assert_line_not_found!(@ $result, $line_location.range.start $(, $($arg)*)?);
        };
        ($result:expr, after: $line_location:expr $(, $($arg:tt)*)?) => {
            let line_location = $line_location;
            $crate::line::tests::assert_line_not_found!(@ $result, line_location.range.end + line_location.line_break_len $(, $($arg)*)?);
        };
        (@ $result:expr, $next_line_index:expr $(, $($arg:tt)*)?) => {
            assert_eq!($result, Err($next_line_index) $(, $($arg)*)?);
        };
    }
    pub(crate) use assert_line_not_found;

    mod binary_search {
        use super::*;

        #[test]
        fn empty() {
            assert_eq!(binary_search(b"a", &[]), Err(0));
        }

        #[test]
        fn lf() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::Lf, true);

            assert_line_found!(binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn crlf() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::CrLf, true);

            assert_line_found!(binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn lf_no_final_line_break() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::Lf, false);

            assert_line_found!(binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn crlf_no_final_line_break() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::CrLf, false);

            assert_line_found!(binary_search("aa", &bytes), locs["aa"]);
            assert_line_found!(binary_search("bb", &bytes), locs["bb"]);
            assert_line_found!(binary_search("cc", &bytes), locs["cc"]);
            assert_line_found!(binary_search("dd", &bytes), locs["dd"]);
            assert_line_found!(binary_search("ee", &bytes), locs["ee"]);
            assert_line_not_found!(binary_search("a", &bytes), before: locs["aa"]);
            assert_line_not_found!(binary_search("b", &bytes), before: locs["bb"]);
            assert_line_not_found!(binary_search("c", &bytes), before: locs["cc"]);
            assert_line_not_found!(binary_search("d", &bytes), before: locs["dd"]);
            assert_line_not_found!(binary_search("e", &bytes), before: locs["ee"]);
            assert_line_not_found!(binary_search("f", &bytes), after: locs["ee"]);
        }

        #[test]
        fn cr_without_lf() {
            let bytes = [b'a', b'a', CR, b'b', b'b'];

            assert_eq!(binary_search("aa", &bytes), Err(0));
            assert_eq!(binary_search("aa\rbb", &bytes), Ok((0..5).into()));
            assert_eq!(binary_search("bb", &bytes), Err(5));
        }

        #[test]
        fn large_line() {
            let (bytes, locs) = make_bytes(["aa", "bb"], LineBreak::Lf, false);

            assert_line_not_found!(binary_search("aaaa", &bytes), after: locs["aa"]);
        }
    }
}
