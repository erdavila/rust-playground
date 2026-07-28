//! Binary search of lines as loaded from a text file.
//!
//! Lines are delimited by line breaks, which are a line feed (`\n`) and may include a preceeding
//! carriage return (`\r`) when present. The last line may not have a line break.

use core::cmp::Ordering;
use core::convert::Infallible;

use crate::ext::{RangeExt as _, SliceExt as _};
use crate::{Comparison, Range, subslice};

#[cfg(feature = "std")]
pub mod buffered;

const CR: u8 = b'\r';
const LF: u8 = b'\n';

/// Executes a binary search of a text line.
///
/// When the `target_line` is found, its byte range without the line break is returned. If the
/// `target_line` is not found then [`Err`] is returned, containing the index where the
/// `target_line` could be inserted while maintaining sorted order.
///
/// To execute a binary search by on-demand loading the content of a file, use
/// [`line::buffered::binary_search`](crate::line::buffered::binary_search).
///
/// This function is also available as an [extension method for slices](crate::ext::ByteSliceExt::line_binary_search).
///
/// # Example
///
/// - [Lines from a text file load in memory](crate#lines-from-a-text-file-in-memory)
#[expect(clippy::missing_errors_doc)]
pub fn binary_search(target_line: impl AsRef<[u8]>, bytes: &[u8]) -> Result<Range, usize> {
    let target_line = target_line.as_ref();
    let Ok(result) = subslice::binary_search_by::<_, Infallible>(bytes, |search_slice| {
        let midpoint = search_slice.range().midpoint();
        let located_line_start = search_slice
            .in_range(..midpoint, |slice| slice.locate_last(|&b| b == LF))
            .map_or(0, |i| i + 1);

        let prefix_len = target_line.len().min(midpoint - located_line_start);
        let line_prefix = &target_line[..prefix_len];
        let located_line_prefix =
            &search_slice[Range::from_start_and_len(located_line_start, prefix_len)];

        // Compare prefix bytes of `line` and the located line.
        let cmp = match located_line_prefix.cmp(line_prefix) {
            Ordering::Less => {
                let next_line_start = search_slice
                    .in_range(midpoint.., |slice| slice.locate_first(|&b| b == LF))
                    .map_or(search_slice.len(), |i| i + 1);
                Comparison::After(next_line_start)
            }
            Ordering::Equal => {
                let mut line_bytes = target_line.iter().skip(prefix_len);
                let mut located_line_bytes = search_slice
                    .iter()
                    .enumerate()
                    .skip(located_line_start + prefix_len)
                    .peekable();

                // Compare remaining bytes.
                loop {
                    let located_line_next_byte = match located_line_bytes.next() {
                        Some((i, &LF)) => {
                            // Located line ended with LF.
                            Err(LineEnd {
                                position: i,
                                next_line_start: i + 1,
                            })
                        }
                        Some((i, &CR)) => {
                            if located_line_bytes.peek().is_some_and(|(_, b)| **b == LF) {
                                // Located line ended with CR + LF.
                                Err(LineEnd {
                                    position: i,
                                    next_line_start: i + 2,
                                })
                            } else {
                                // Located line has CR not followed by LF.
                                Ok(CR)
                            }
                        }
                        Some((_, &b)) => {
                            // Regular byte in the located line.
                            Ok(b)
                        }
                        None => {
                            // Located Line ended without a line break.
                            Err(LineEnd {
                                position: search_slice.len(),
                                next_line_start: search_slice.len(),
                            })
                        }
                    };

                    match (located_line_next_byte, line_bytes.next()) {
                        (Ok(_), None) => {
                            // Located line is longer than the `line`.
                            break Comparison::Before(located_line_start);
                        }
                        (Err(e), None) => {
                            // Both `line` and the located ended with equal bytes.
                            break Comparison::Found((located_line_start..e.position).into());
                        }
                        (Ok(loc_b), Some(b)) => match loc_b.cmp(b) {
                            Ordering::Less => {
                                // Stop the comparison.
                                // Next round will search after the located line.
                                let next_line_start = located_line_bytes
                                    .find_map(|(i, &b)| (b == LF).then_some(i + 1))
                                    .unwrap_or(search_slice.len());
                                break Comparison::After(next_line_start);
                            }
                            Ordering::Equal => {
                                // Bytes in `line` and in the located line are equal.
                                // Continue comparing.
                            }
                            Ordering::Greater => {
                                // Stop the comparison.
                                // Next round will search before the located line.
                                break Comparison::Before(located_line_start);
                            }
                        },
                        (Err(e), Some(_)) => {
                            // `line` is longer than the located line.
                            break Comparison::After(e.next_line_start);
                        }
                    }
                }
            }
            Ordering::Greater => Comparison::Before(located_line_start),
        };

        Ok(Some(cmp))
    });
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LineEnd {
    position: usize,
    next_line_start: usize,
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
