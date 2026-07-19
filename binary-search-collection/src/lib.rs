#![no_std]

use core::cmp::Ordering;
use core::range::Range;

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
pub fn line_binary_search(
    target_line: impl AsRef<[u8]>,
    bytes: &[u8],
) -> Result<Range<usize>, usize> {
    let target_line = target_line.as_ref();

    let mut start = 0;
    let mut end = bytes.len();

    while start < end {
        let mid = start.midpoint(end);

        let line_start = bytes[start..mid]
            .iter()
            .rposition(|&b| b == LF)
            .map_or(start, |i| i + 1 + start);

        let break_end = bytes[mid..end]
            .iter()
            .position(|&b| b == LF)
            .map_or(end, |i| i + 1 + mid);

        let line_without_line_break = strip_line_break(&bytes[line_start..break_end]);

        match line_without_line_break.cmp(target_line) {
            Ordering::Less => start = break_end,
            Ordering::Equal => {
                let line_end = line_start + line_without_line_break.len();
                return Ok((line_start..line_end).into());
            }
            Ordering::Greater => end = line_start,
        }
    }

    Err(start)
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
            range: Range<usize>,
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
}
