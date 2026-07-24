//! Binary search of lines as loaded from a text file.

use crate::ext::{RangeExt as _, SliceExt as _};
use crate::subslice::LocatedSubslice;
use crate::{Range, SearchResult, subslice};

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
/// use binary_search_collection::Range;
/// use binary_search_collection::line;
///
/// fn locate_line_in_file<P: AsRef<Path>>(line: &str, path: P) -> Option<Range> {
///     let bytes = read(path).unwrap();
///     line::binary_search(line, &bytes).ok()
/// }
/// ```
#[expect(clippy::missing_errors_doc)]
pub fn binary_search(target_line: impl AsRef<[u8]>, bytes: &[u8]) -> SearchResult<Range> {
    subslice::binary_search(target_line.as_ref(), bytes, |search_slice| {
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

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    mod binary_search {
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
            assert_eq!(binary_search(b"a", &[]), Err(0));
        }

        #[test]
        fn lf() {
            let (bytes, locs) = make_bytes(["aa", "bb", "cc", "dd", "ee"], LineBreak::Lf, true);
            assert_eq!(bytes.len(), 15);

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
            assert_eq!(bytes.len(), 20);

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
            assert_eq!(bytes.len(), 14);

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
            assert_eq!(bytes.len(), 18);

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
}
