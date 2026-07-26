use std::collections::VecDeque;
use std::fmt::Debug;
use std::num::{NonZeroUsize, TryFromIntError};
use std::{cmp, io};

use crate::ext::{ByteSliceExt, RangeExt as _, SliceExt as _};
use crate::line::{CR, LF};
use crate::{LocatedItem, Range, SearchResult, sparse};

fn convert_int<From, To>(from: From) -> io::Result<To>
where
    From: TryInto<To, Error = TryFromIntError>,
{
    from.try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::FileTooLarge, e))
}

/// Executes a binary search of a text line in (possibly) a [`File`].
///
/// The source can be anything that implements [`Read`] and [`Seek`], which includes
/// [`File`] and [`Cursor`].
///
/// The `buffered_len` parameter is the preferred and maximum amount of bytes that are read each time.
///
/// When the `target_line` line is found, its byte range without the line break is returned. If the
/// target `target_line` is not found then [`Err`] is returned, containing the index where the
/// `target_line` could be inserted while maintaining sorted order.
///
/// To execute binary search of a file content loaded into memory, use [`line::binary_search`](crate::line::binary_search).
///
/// # Example
///
/// - [Lines in a text file](crate#lines-in-a-text-file)
///
/// [`Cursor`]: std::io::Cursor
/// [`File`]: std::fs::File
/// [`Read`]: std::io::Read
/// [`Seek`]: std::io::Seek
///
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<S: io::Read + io::Seek>(
    target_line: impl AsRef<[u8]>,
    mut source: S,
    buffer_len: NonZeroUsize,
) -> SearchResult<Range, io::Error> {
    let element_count: usize = convert_int(source.seek(io::SeekFrom::End(0))?)?;
    let buffer_len = cmp::min(element_count, buffer_len.get());

    let mut reader = Reader::new(source);

    sparse::binary_search(target_line.as_ref(), element_count, |search_range| {
        let buffer_len = cmp::min(buffer_len, search_range.len());
        let midpoint = search_range.midpoint();
        let read_start = {
            let s = midpoint.saturating_sub(buffer_len / 2);
            s.clamp(
                search_range.start,
                search_range.end.saturating_sub(buffer_len),
            )
        };
        let midpoint_index = midpoint - read_start;

        let mut line = Line::new(midpoint, midpoint_index, buffer_len);
        let buffer = line.full_slice();

        reader.read(read_start, buffer)?;

        let ls = buffer.extend_subslice_range_to_delimiters(
            Range::from_start_and_len(midpoint_index, 0),
            |&b| b == LF,
        );

        // Line start LF is not included.
        line.incorporate_front_buffer_bytes(midpoint_index - ls.subslice_range.start);
        // Line end LF is included, if found.
        line.incorporate_back_buffer_bytes(ls.consumed_range.end - midpoint_index);

        let start_lf_found = ls.consumed_range.start < ls.subslice_range.start;
        if !start_lf_found {
            while line.start() > search_range.start {
                let read_len = cmp::min(buffer_len, line.start() - search_range.start);
                let read_start = line.start() - read_len;
                let buffer = line.set_front_buffer_len(read_len);

                reader.read(read_start, buffer)?;

                if let Some(i) = buffer.locate_last(|&b| b == LF).map(|i| i + 1) {
                    // Incorporates the bytes after LF.
                    line.incorporate_front_buffer_bytes(read_len - i);
                    break;
                }

                line.incorporate_front_buffer_bytes(read_len);
            }
        }
        line.set_front_buffer_len(0);

        let end_lf_found = ls.consumed_range.end > ls.subslice_range.end;
        if !end_lf_found {
            while line.end() < search_range.end {
                let read_len = cmp::min(buffer_len, search_range.end - line.end());
                let read_start = line.end();
                let buffer = line.set_back_buffer_len(read_len);

                reader.read(read_start, buffer)?;

                if let Some(i) = buffer.locate_first(|&b| b == LF).map(|i| i + 1) {
                    // Incorporates the bytes up to (and including) LF.
                    line.incorporate_back_buffer_bytes(i);
                    break;
                }

                line.incorporate_back_buffer_bytes(read_len);
            }
        }
        line.set_back_buffer_len(0);

        let (value, value_start, line_break_len) = line.into_bytes_and_start();

        let value_range = Range::from_start_and_len(value_start, value.len());
        let mut consumed_range = value_range;
        consumed_range.end += line_break_len;

        Ok(Some(LocatedItem {
            value,
            value_range,
            consumed_range,
        }))
    })
}

struct Reader<S> {
    source: S,
}

impl<S> Reader<S> {
    fn new(source: S) -> Self {
        Reader { source }
    }

    fn read(&mut self, start: usize, buffer: &mut [u8]) -> io::Result<()>
    where
        S: io::Read + io::Seek,
    {
        assert!(!buffer.is_empty());
        self.source.seek(io::SeekFrom::Start(convert_int(start)?))?;
        self.source.read_exact(buffer)?;
        Ok(())
    }
}

struct Line {
    bytes: VecDeque<u8>,
    start: usize,
    front_buffer_len: usize,
    back_buffer_len: usize,
}

impl Line {
    fn new(midpoint: usize, midpoint_index: usize, len: usize) -> Self {
        assert!(midpoint_index < len);

        let mut line = Line {
            bytes: VecDeque::new(),
            start: midpoint,
            front_buffer_len: midpoint_index,
            back_buffer_len: len - midpoint_index,
        };

        line.increase_len(len);
        line
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.start() + self.len()
    }

    fn len(&self) -> usize {
        self.bytes.len() - self.front_buffer_len - self.back_buffer_len
    }

    fn incorporate_front_buffer_bytes(&mut self, count: usize) {
        assert!(count <= self.front_buffer_len);
        self.start -= count;
        self.front_buffer_len -= count;
    }

    fn incorporate_back_buffer_bytes(&mut self, count: usize) {
        assert!(count <= self.back_buffer_len);
        self.back_buffer_len -= count;
    }

    fn set_front_buffer_len(&mut self, len: usize) -> &mut [u8] {
        match len.cmp(&self.front_buffer_len) {
            cmp::Ordering::Less => {
                let decrease = self.front_buffer_len - len;
                self.bytes.rotate_left(decrease);
                self.decrease_len(decrease);
                self.front_buffer_len -= decrease;
            }
            cmp::Ordering::Equal => {}
            cmp::Ordering::Greater => {
                let increase = len - self.front_buffer_len;
                self.increase_len(increase);
                self.bytes.rotate_right(increase);
                self.front_buffer_len += increase;
            }
        }
        self.slice(Range::from_start_and_len(0, self.front_buffer_len))
    }

    fn set_back_buffer_len(&mut self, len: usize) -> &mut [u8] {
        match len.cmp(&self.back_buffer_len) {
            cmp::Ordering::Less => {
                let decrease = self.back_buffer_len - len;
                self.decrease_len(decrease);
                self.back_buffer_len -= decrease;
            }
            cmp::Ordering::Equal => {}
            cmp::Ordering::Greater => {
                let increase = len - self.back_buffer_len;
                self.increase_len(increase);
                self.back_buffer_len += increase;
            }
        }
        self.slice(Range::from_start_and_len(
            self.bytes.len() - self.back_buffer_len,
            self.back_buffer_len,
        ))
    }

    fn increase_len(&mut self, count: usize) {
        self.bytes.resize(self.bytes.len() + count, b'\0');
    }

    fn decrease_len(&mut self, count: usize) {
        self.bytes.truncate(self.bytes.len() - count);
    }

    fn full_slice(&mut self) -> &mut [u8] {
        self.bytes.make_contiguous()
    }

    fn slice(&mut self, range: Range) -> &mut [u8] {
        /*
            Ideally, the compiler should accept this:

        let (front, back) = vd.as_mut_slices();
        if range.end <= front.len() {
            &mut front[range]
        } else if range.start >= front.len()  {
            let range = Range::from_start_and_len(range.start - front.len(), range.len());
            &mut back[range]
        } else {
            &mut vd.make_contiguous()[range]
        }

        */

        let front_len = self.bytes.as_slices().0.len();
        if range.end < front_len {
            let (front, _) = self.bytes.as_mut_slices();
            &mut front[range]
        } else if range.start >= front_len {
            let (_, back) = self.bytes.as_mut_slices();
            let range = Range::from_start_and_len(range.start - front_len, range.len());
            &mut back[range]
        } else {
            &mut self.bytes.make_contiguous()[range]
        }
    }

    fn into_bytes_and_start(mut self) -> (Vec<u8>, usize, usize) {
        self.bytes.rotate_left(self.front_buffer_len);
        self.decrease_len(self.front_buffer_len + self.back_buffer_len);

        let mut line_break_len = 0;
        if self.bytes.pop_back_if(|b| *b == LF).is_some() {
            line_break_len += 1;
            if self.bytes.pop_back_if(|b| *b == CR).is_some() {
                line_break_len += 1;
            }
        }

        let value = self.bytes.into();
        (value, self.start, line_break_len)
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes: Vec<_> = self.bytes.iter().copied().collect();
        f.debug_struct("Line")
            .field("bytes", &bytes.debug_str())
            .field("start", &self.start)
            .field("front_buffer_len", &self.front_buffer_len)
            .field("back_buffer_len", &self.back_buffer_len)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod binary_search {
        use std::io::Cursor;

        use super::*;
        use crate::line::tests::{LineBreak, make_bytes};

        macro_rules! assert_line_found {
            ($result:expr, $line_location:expr, $($arg:tt)*) => {
                let result = $result
                    .map_err(|e| format!("{e}: {}", format_args!($($arg)*)))
                    .unwrap();
                $crate::line::tests::assert_line_found!(result, $line_location, $($arg)*);
            };
        }

        macro_rules! assert_line_not_found {
            ($result:expr, $rel:tt : $line_location:expr $(, $($arg:tt)*)?) => {
                let result = $result
                    .map_err(|e| format!("{e}: {}", format_args!($($($arg)*)?)))
                    .unwrap();
                $crate::line::tests::assert_line_not_found!(result, $rel: $line_location $(, $($arg)*)?);
            };
        }

        static LINES: [&str; 5] = ["abcd", "efgh", "ijkl", "mnop", "qrst"];

        macro_rules! test_case {
            ($source:expr, $buffer_len:expr, $locs:expr, $($arg:tt)+) => {
                let mut source = $source;
                let buffer_len = $buffer_len;
                let locs = $locs;

                for line in LINES {
                    let target = &line[..2];
                    assert_line_not_found!(
                        binary_search(target, &mut source, buffer_len),
                        before: locs[line],
                        "expecting not found: target={target:?}, {}", format_args!($($arg)+)
                    );

                    let target = line;
                    assert_line_found!(
                        binary_search(target, &mut source, buffer_len),
                        locs[line],
                        "expecting found: target={target:?}, {}", format_args!($($arg)+)
                    );
                }

                let target = "uv";
                assert_line_not_found!(
                    binary_search(target, &mut source, buffer_len),
                    after: locs["qrst"],
                    "expecting not found: target={target:?}, {}", format_args!($($arg)+)
                );
            };
        }

        #[test]
        fn empty() {
            let mut source = Cursor::new(&[]);
            let buffer_len = 1.try_into().unwrap();
            let result = binary_search("a", &mut source, buffer_len).unwrap();
            assert_eq!(result, Err(0));
        }

        #[test]
        fn non_empty() {
            for line_break in [LineBreak::Lf, LineBreak::CrLf] {
                for final_line_break in [true, false] {
                    let (bytes, locs) = make_bytes(LINES, line_break, final_line_break);
                    let mut source = Cursor::new(&bytes);

                    for buffer_len in 1..=bytes.len() + 1 {
                        let buffer_len = buffer_len.try_into().unwrap();
                        test_case!(
                            &mut source,
                            buffer_len,
                            &locs,
                            "line_break={line_break:?}, final_line_break={final_line_break}, buffer_len={buffer_len}"
                        );
                    }
                }
            }
        }
    }
}
