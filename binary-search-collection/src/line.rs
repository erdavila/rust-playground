//! Binary search of lines as loaded from a text file.
//!
//! Lines are delimited by line breaks, which are a line feed (`\n`) and may include a preceeding
//! carriage return (`\r`) when present. The last line may not have a line break.

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

// Implementation using `line::binary_search_by`.
#[cfg(not(feature = "alternative-line-binary_search"))]
mod binary_search {
    use core::cmp::Ordering;
    use core::convert::Infallible;

    use crate::{Range, line};

    pub(super) fn implementation(
        target_line: impl AsRef<[u8]>,
        bytes: &[u8],
    ) -> Result<Range, usize> {
        let target_line = target_line.as_ref();
        let Ok(result) = line::binary_search_by::<Infallible>(bytes, |located_line_bytes| {
            let located_line_prefix = located_line_bytes.next_slice(target_line.len());

            let cmp = located_line_prefix.cmp(target_line).then_with(|| {
                // The located line has `line` as a prefix.
                if located_line_bytes.next().is_some() {
                    // The located line has more bytes than `line`.
                    Ordering::Greater
                } else {
                    // The located line and the `line` are equal.
                    Ordering::Equal
                }
            });

            Ok(cmp)
        });

        result
    }
}

// Implementation using `subslice::binary_search_by`.
#[cfg(feature = "alternative-line-binary_search")]
mod binary_search {
    use core::cmp::Ordering;
    use core::convert::Infallible;

    use crate::ext::{ByteSliceExt, RangeExt as _, SliceExt as _};
    use crate::line::LineBytes;
    use crate::{Comparison, Range, subslice};

    pub(super) fn implementation(
        target_line: impl AsRef<[u8]>,
        bytes: &[u8],
    ) -> Result<Range, usize> {
        let target_line = target_line.as_ref();
        let Ok(result) = subslice::binary_search_by::<_, Infallible>(bytes, |search_slice| {
            let midpoint = search_slice.range().midpoint();
            let located_line_start =
                search_slice.in_range(..midpoint, ByteSliceExt::locate_line_start_from_end_or_zero);

            let mut located_line_bytes = LineBytes::new(search_slice, located_line_start);
            let located_line_prefix = located_line_bytes.next_slice(target_line.len());

            let cmp = match located_line_prefix.cmp(target_line) {
                Ordering::Less => {
                    let line_end = located_line_bytes.skip_to_end();
                    let next_line_start = line_end.line_break_end_position();
                    Comparison::After(next_line_start)
                }
                Ordering::Equal => {
                    // The located line has `line` as a prefix.
                    if located_line_bytes.next().is_some() {
                        // The located line has more bytes than `line`.
                        Comparison::Before(located_line_start)
                    } else {
                        // The located line and the `line` are equal.
                        let line_end = located_line_bytes.skip_to_end();
                        let located_line_end = line_end.position;
                        Comparison::Found((located_line_start..located_line_end).into())
                    }
                }
                Ordering::Greater => Comparison::Before(located_line_start),
            };

            Ok(Some(cmp))
        });
        result
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
        let midpoint = search_slice.range().midpoint();
        let start =
            search_slice.in_range(..midpoint, ByteSliceExt::locate_line_start_from_end_or_zero);

        let mut line_bytes = LineBytes::new(search_slice, start);

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
    /// Creates a new instance.
    ///
    /// The line must be located at the start of the `bytes` slice.
    ///
    /// The `bytes` slice must contain at least the full line, and may include the line break and
    /// additional bytes. The additional bytes are ignored.
    #[must_use]
    pub fn new(bytes: &'a [u8], start_index: usize) -> Self {
        LineBytes {
            bytes,
            next_index: start_index,
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
