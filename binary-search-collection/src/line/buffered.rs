use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{self, Debug};
use std::num::{NonZeroUsize, TryFromIntError};
use std::{cmp, io, mem};

use crate::ext::{ByteSliceExt, RangeExt as _, SliceExt as _};
use crate::line::{CR, DebugStr, LF, LineEnd, buffered};
use crate::{Comparison, Range, SearchResult, sparse};

fn convert_int<From, To>(from: From) -> io::Result<To>
where
    From: TryInto<To, Error = TryFromIntError>,
{
    from.try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::FileTooLarge, e))
}

/// Executes a binary search of a text line in (possibly) a [`File`].
///
/// The `source` can be anything that implements [`Read`] and [`Seek`], which includes [`File`] and
/// [`Cursor`].
///
/// The `buffered_len` parameter is the preferred and maximum amount of bytes that are read each time.
///
/// When the `target_line` line is found, its byte range without the line break is returned. If the
/// target `target_line` is not found then [`Err`] is returned, containing the index where the
/// `target_line` could be inserted while maintaining sorted order.
///
/// To execute binary search of a file content loaded into memory, use [`line::binary_search`].
///
/// # Example
///
/// - [Lines in a text file](crate#lines-in-a-text-file)
///
/// [`Cursor`]: std::io::Cursor
/// [`File`]: std::fs::File
/// [`Read`]: std::io::Read
/// [`Seek`]: std::io::Seek
#[expect(clippy::missing_errors_doc)]
pub fn binary_search<S>(
    target_line: impl AsRef<[u8]>,
    source: S,
    buffer_len: NonZeroUsize,
) -> SearchResult<Range, io::Error>
where
    S: io::Read + io::Seek,
{
    binary_search::implementation(target_line, source, buffer_len)
}

// Implementation using `line::buffered::binary_search_by`.
#[cfg(not(any(
    feature = "alternative-line_buffered-binary_search-1",
    feature = "alternative-line_buffered-binary_search-2",
)))]
mod binary_search {
    use std::cmp::Ordering;
    use std::num::NonZeroUsize;
    use std::{cmp, io};

    use crate::{Range, SearchResult, line};

    pub(super) fn implementation<S>(
        target_line: impl AsRef<[u8]>,
        source: S,
        buffer_len: NonZeroUsize,
    ) -> SearchResult<Range, io::Error>
    where
        S: io::Read + io::Seek,
    {
        line::buffered::binary_search_by(source, buffer_len, |loc_line_bytes| {
            let mut target_line = target_line.as_ref();

            while !target_line.is_empty() {
                let Some(chunk) = loc_line_bytes.next_chunk()? else {
                    return Ok(Ordering::Less);
                };

                let cmp_len = cmp::min(target_line.len(), chunk.len());
                let (chunk_prefix, chunk_rest) = chunk.split_at(cmp_len);
                let (line_prefix, line_rest) = target_line.split_at(cmp_len);

                let cmp = chunk_prefix.cmp(line_prefix);
                if cmp.is_ne() {
                    return Ok(cmp);
                }

                if !chunk_rest.is_empty() {
                    // The chunk has additional bytes.
                    return Ok(Ordering::Greater);
                }

                target_line = line_rest;
            }

            if loc_line_bytes.next().is_some() {
                Ok(Ordering::Greater)
            } else {
                Ok(Ordering::Equal)
            }
        })
    }
}

// Implementation using `sparse::binary_search`.
#[cfg(feature = "alternative-line_buffered-binary_search-1")]
mod binary_search {
    use std::num::NonZeroUsize;
    use std::{cmp, io};

    use crate::ext::{RangeExt as _, SliceExt as _};
    use crate::line::LF;
    use crate::line::buffered::{Line, Reader, convert_int};
    use crate::{LocatedItem, Range, SearchResult, sparse};

    pub(super) fn implementation<S>(
        target_line: impl AsRef<[u8]>,
        mut source: S,
        buffer_len: NonZeroUsize,
    ) -> SearchResult<Range, io::Error>
    where
        S: io::Read + io::Seek,
    {
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
}

// Implementation using `line::buffered::binary_search_by_key`.
#[cfg(feature = "alternative-line_buffered-binary_search-2")]
mod binary_search {
    use std::borrow::Cow;
    use std::io;
    use std::num::NonZeroUsize;

    use crate::{Range, SearchResult, line};

    pub(super) fn implementation<S>(
        target_line: impl AsRef<[u8]>,
        source: S,
        buffer_len: NonZeroUsize,
    ) -> SearchResult<Range, io::Error>
    where
        S: io::Read + io::Seek,
    {
        let target = Cow::Borrowed(target_line.as_ref());
        line::buffered::binary_search_by_key(&target, source, buffer_len, |line_bytes| {
            let vec = line_bytes.collect::<io::Result<Vec<_>>>()?;
            Ok(Cow::Owned(vec))
        })
    }
}

/// Executes a binary search of a text line in (possibly) a [`File`] with a comparison key
/// extraction.
///
/// The `source` can be anything that implements [`Read`] and [`Seek`], which includes [`File`] and
/// [`Cursor`].
///
/// The `buffered_len` parameter is the preferred and maximum amount of bytes that are read each time.
///
/// The `extract` closure must extract from its [`BufferedLineBytes`] parameter the value to be
/// compared against the `target`.
///
/// When the target line is found, its byte range without the line break is returned. If the target
/// line is not found then [`Err`] is returned, containing the index where the target line could be
/// inserted while maintaining sorted order.
///
/// [`Cursor`]: std::io::Cursor
/// [`File`]: std::fs::File
/// [`Read`]: std::io::Read
/// [`Seek`]: std::io::Seek
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by_key<T, S, E>(
    target: &T,
    source: S,
    buffer_len: NonZeroUsize,
    mut extract: impl FnMut(&mut BufferedLineBytes<S>) -> Result<T, E>,
) -> SearchResult<Range, E>
where
    T: Ord,
    S: io::Read + io::Seek,
    E: From<io::Error>,
{
    buffered::binary_search_by(source, buffer_len, |line_bytes| {
        let value = extract(line_bytes)?;
        let cmp = value.cmp(target);
        Ok(cmp)
    })
}

/// Executes a binary search of a text line in (possibly) a [`File`] with a custom comparison.
///
/// The `source` can be anything that implements [`Read`] and [`Seek`], which includes [`File`] and
/// [`Cursor`].
///
/// The `buffered_len` parameter is the preferred and maximum amount of bytes that are read each time.
///
/// The `compare` closure must read the bytes in its [`BufferedLineBytes`] parameter and compare it
/// with the target line, returning the proper [`Ordering`] value.
///
/// When the target line is found, its byte range without the line break is returned. If the target
/// line is not found then [`Err`] is returned, containing the index where the target line could be
/// inserted while maintaining sorted order.
///
/// To execute binary search of a file content loaded into memory with a custom comparison, use
/// [`line::binary_search_by`](crate::line::binary_search).
///
/// [`Cursor`]: std::io::Cursor
/// [`File`]: std::fs::File
/// [`Read`]: std::io::Read
/// [`Seek`]: std::io::Seek
#[expect(clippy::missing_errors_doc)]
pub fn binary_search_by<S, E>(
    mut source: S,
    buffer_len: NonZeroUsize,
    mut compare: impl FnMut(&mut BufferedLineBytes<S>) -> Result<Ordering, E>,
) -> SearchResult<Range, E>
where
    S: io::Read + io::Seek,
    E: From<io::Error>,
{
    let element_count = convert_int(source.seek(io::SeekFrom::End(0))?)?;
    let buffer_len = cmp::min(element_count, buffer_len.get());
    sparse::binary_search_by(element_count, |search_range| {
        let mut reader = ChunkReader::new(&mut source, search_range, buffer_len);
        let (mut mid_chunk, mut start_position, midpoint_index) =
            reader.read_chunk_around_midpoint()?;

        let start_index =
            mid_chunk.in_range(..midpoint_index, ByteSliceExt::locate_line_start_from_end);
        let mut start_chunks = Vec::new();

        if let Some(start_index) = start_index {
            // Start LF found in the mid chunk.
            start_position += start_index;
            mid_chunk.splice(..start_index, []);
        } else {
            // Start LF not found in the mid chunk.
            while let Some(chunk) = reader.read_chunk_with_end_at(start_position)? {
                start_position -= chunk.len();

                if let Some(i) = chunk.locate_line_start_from_end() {
                    // Start LF found in the start chunk.
                    let mut chunk = VecDeque::from(chunk);
                    chunk.rotate_left(i);
                    chunk.truncate(chunk.len() - i);
                    if !chunk.is_empty() {
                        start_chunks.push(chunk);
                    }
                    start_position += i;
                    break;
                }

                start_chunks.push(chunk.into());
            }
        }

        let mut line_bytes =
            BufferedLineBytes::new(start_position, start_chunks, mid_chunk.into(), reader);

        let cmp = match compare(&mut line_bytes)? {
            Ordering::Less => {
                let line_end = line_bytes.skip_to_end()?;
                Comparison::After(line_end.line_break_end_position())
            }
            Ordering::Equal => {
                let line_end = line_bytes.skip_to_end()?;
                Comparison::Found((start_position..line_end.position).into())
            }
            Ordering::Greater => Comparison::Before(start_position),
        };

        Ok(Some(cmp))
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

#[cfg(feature = "alternative-line_buffered-binary_search-1")]
struct Line {
    bytes: VecDeque<u8>,
    start: usize,
    front_buffer_len: usize,
    back_buffer_len: usize,
}

#[cfg(feature = "alternative-line_buffered-binary_search-1")]
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

#[cfg(feature = "alternative-line_buffered-binary_search-1")]
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

pub(crate) struct ChunkReader<'a, S> {
    reader: Reader<&'a mut S>,
    range: Range,
    buffer_len: usize,
}

impl<'a, S> ChunkReader<'a, S>
where
    S: io::Read + io::Seek,
{
    pub(crate) fn new(source: &'a mut S, range: Range, buffer_len: usize) -> Self {
        ChunkReader {
            reader: Reader::new(source),
            range,
            buffer_len: cmp::min(buffer_len, range.len()),
        }
    }

    pub(crate) fn read_chunk_around_midpoint(&mut self) -> io::Result<(Vec<u8>, usize, usize)> {
        let midpoint = self.range.midpoint();
        let start = midpoint.saturating_sub(self.buffer_len / 2).clamp(
            self.range.start,
            self.range.end.saturating_sub(self.buffer_len),
        );
        let midpoint_index = midpoint - start;
        let chunk = self.read_chunk(start, self.buffer_len)?;
        Ok((chunk, start, midpoint_index))
    }

    pub(crate) fn read_chunk_with_end_at(&mut self, end: usize) -> io::Result<Option<Vec<u8>>> {
        (end > self.range.start)
            .then(|| {
                let len = cmp::min(self.buffer_len, end - self.range.start);
                let start = end - len;
                self.read_chunk(start, len)
            })
            .transpose()
    }

    pub(crate) fn read_chunk_with_start_at(&mut self, start: usize) -> io::Result<Option<Vec<u8>>> {
        (start < self.range.end)
            .then(|| {
                let len = cmp::min(self.buffer_len, self.range.end - start);
                self.read_chunk(start, len)
            })
            .transpose()
    }

    fn read_chunk(&mut self, start: usize, len: usize) -> io::Result<Vec<u8>> {
        let mut chunk = Vec::with_capacity(len);
        chunk.resize(len, b'\0');
        self.reader.read(start, &mut chunk)?;
        Ok(chunk)
    }
}

impl<S> Debug for ChunkReader<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkReader")
            .field("reader", &fmt::from_fn(|f| write!(f, "...")))
            .field("range", &self.range)
            .field("buffer_len", &self.buffer_len)
            .finish()
    }
}

struct BufLineBytesStart<'a, S> {
    // Stack of chunks before the midpoint. Don't contain LF.
    chunks: Vec<VecDeque<u8>>,
    // Must be `None` only right before dropping self.
    mid_to_end: Option<BufLineBytesMidToEnd<'a, S>>,
}

impl<S> Debug for BufLineBytesStart<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufLineBytesStart")
            .field(
                "chunks",
                &fmt::from_fn(|f| {
                    f.debug_list()
                        .entries(self.chunks.iter().rev().map(DebugStr::debug_str))
                        .finish()
                }),
            )
            .field("mid_to_end", &self.mid_to_end)
            .finish()
    }
}

struct BufLineBytesMidToEnd<'a, S> {
    // May contain LF. May be empty.
    chunk: VecDeque<u8>,
    reader: ChunkReader<'a, S>,
}

impl<S> Debug for BufLineBytesMidToEnd<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufLineBytesMidToEnd")
            .field("chunk", &self.chunk.debug_str())
            .field("reader", &self.reader)
            .finish()
    }
}

enum BufLineBytesState<'a, S> {
    Start(BufLineBytesStart<'a, S>),
    MidToEnd(BufLineBytesMidToEnd<'a, S>),
    End { line_break_len: usize },
}

impl<S> Debug for BufLineBytesState<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start(start) => f.debug_tuple("Start").field(start).finish(),
            Self::MidToEnd(mid_to_end) => f.debug_tuple("MidToEnd").field(mid_to_end).finish(),
            Self::End { line_break_len } => f
                .debug_struct("End")
                .field("line_break_len", line_break_len)
                .finish(),
        }
    }
}

pub struct BufferedLineBytes<'a, S> {
    next_position: usize,
    state: BufLineBytesState<'a, S>,
}

impl<'a, S> BufferedLineBytes<'a, S> {
    pub(crate) fn new(
        start_position: usize,
        start_chunks: Vec<VecDeque<u8>>,
        mid_chunk: VecDeque<u8>,
        reader: ChunkReader<'a, S>,
    ) -> Self {
        BufferedLineBytes {
            next_position: start_position,
            state: BufLineBytesState::Start(BufLineBytesStart {
                chunks: start_chunks,
                mid_to_end: Some(BufLineBytesMidToEnd {
                    chunk: mid_chunk,
                    reader,
                }),
            }),
        }
    }

    fn next_byte(&mut self) -> io::Result<Option<u8>>
    where
        S: io::Read + io::Seek,
    {
        let output: Option<u8> = match &mut self.state {
            BufLineBytesState::Start(start) => {
                let start_chunks_len = start.chunks.len();
                if let Some(chunk) = start.chunks.last_mut() {
                    match chunk.pop_front() {
                        Some(CR) if chunk.is_empty() && start_chunks_len == 1 => {
                            // CR found as the last byte in the `start` chunks.
                            // Need to check if `mid_to_end` chunk starts with LF.
                            let mid_to_end = start.mid_to_end.take().expect("can't be None");
                            if mid_to_end.chunk[0] == LF {
                                self.state = BufLineBytesState::End { line_break_len: 2 };
                                None
                            } else {
                                self.state = BufLineBytesState::MidToEnd(mid_to_end);
                                Some(CR)
                            }
                        }
                        Some(b) => Some(b),
                        None => {
                            // Drop the empty chunk.
                            start.chunks.pop();
                            // Try again.
                            return self.next_byte();
                        }
                    }
                } else {
                    // No more start chunks.
                    let mid_to_end = start.mid_to_end.take().expect("can't be None");
                    self.state = BufLineBytesState::MidToEnd(mid_to_end);
                    // Try again.
                    return self.next_byte();
                }
            }
            BufLineBytesState::MidToEnd(mid_to_end) => {
                match mid_to_end.chunk.pop_front() {
                    Some(LF) => {
                        self.state = BufLineBytesState::End { line_break_len: 1 };
                        None
                    }
                    Some(CR) => {
                        match mid_to_end.chunk.front() {
                            Some(&LF) => {
                                self.state = BufLineBytesState::End { line_break_len: 2 };
                                None
                            }
                            Some(&_) => Some(CR),
                            None => {
                                // CR was the only byte in the chunk.
                                // Need to check if next chunk starts with LF.
                                let next_chunk_start = self.next_position + 1;
                                if let Some(next_chunk) = mid_to_end
                                    .reader
                                    .read_chunk_with_start_at(next_chunk_start)?
                                {
                                    if next_chunk[0] == LF {
                                        self.state = BufLineBytesState::End { line_break_len: 2 };
                                        None
                                    } else {
                                        mid_to_end.chunk = next_chunk.into();
                                        Some(CR)
                                    }
                                } else {
                                    // Reached the end without an LF.
                                    self.state = BufLineBytesState::End { line_break_len: 2 };
                                    Some(CR)
                                }
                            }
                        }
                    }
                    Some(b) => Some(b),
                    None => {
                        // The chunk is empty.
                        if let Some(chunk) = mid_to_end
                            .reader
                            .read_chunk_with_start_at(self.next_position)?
                        {
                            mid_to_end.chunk = chunk.into();
                            // Try again.
                            return self.next_byte();
                        }

                        // Reached the end without an LF.
                        assert_eq!(self.next_position, mid_to_end.reader.range.end);
                        self.state = BufLineBytesState::End { line_break_len: 0 };
                        None
                    }
                }
            }
            BufLineBytesState::End { .. } => None,
        };

        if output.is_some() {
            self.next_position += 1;
        }

        Ok(output)
    }

    #[expect(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    /// Returns the next non-empty chunk.
    pub fn next_chunk(&mut self) -> io::Result<Option<Vec<u8>>>
    where
        S: io::Read + io::Seek,
    {
        let chunk: Option<Vec<_>> = match &mut self.state {
            BufLineBytesState::Start(start) => {
                if let Some(mut chunk) = start.chunks.pop() {
                    if chunk.is_empty() {
                        // Try again.
                        return self.next_chunk();
                    } else if chunk.back() == Some(&CR) && start.chunks.is_empty() {
                        // It was the last start chunk and it ends with CR.
                        // Need to check if the `mid_to_end` chunk starts with LF.
                        let mid_to_end = start.mid_to_end.as_mut().expect("can't be None");
                        if mid_to_end.chunk[0] == LF {
                            self.state = BufLineBytesState::End { line_break_len: 2 };
                            // Drop the CR.
                            chunk.pop_back();
                            (!chunk.is_empty()).then(|| chunk.into())
                        } else {
                            Some(chunk.into())
                        }
                    } else {
                        Some(chunk.into())
                    }
                } else {
                    // No more start chunks.
                    let mid_to_end = start.mid_to_end.take().expect("can't be None");
                    self.state = BufLineBytesState::MidToEnd(mid_to_end);
                    // Try again.
                    return self.next_chunk();
                }
            }
            BufLineBytesState::MidToEnd(mid_to_end) => {
                let mut chunk: Vec<_> = mem::take(&mut mid_to_end.chunk).into();
                if chunk.is_empty() {
                    if let Some(chunk) = mid_to_end
                        .reader
                        .read_chunk_with_start_at(self.next_position)?
                    {
                        mid_to_end.chunk = chunk.into();
                        // Try again.
                        return self.next_chunk();
                    }

                    // Reached the end without an LF.
                    assert_eq!(self.next_position, mid_to_end.reader.range.end);
                    self.state = BufLineBytesState::End { line_break_len: 0 };
                    None
                } else if let Some(line_end) = chunk.locate_line_end_from_start() {
                    self.state = BufLineBytesState::End {
                        line_break_len: line_end.line_break_len,
                    };
                    chunk.truncate(line_end.position);
                    (!chunk.is_empty()).then_some(chunk)
                } else if chunk.last() == Some(&CR) {
                    // Need to check if next chunk starts with LF.
                    let next_chunk_start = self.next_position + chunk.len();
                    if let Some(next_chunk) = mid_to_end
                        .reader
                        .read_chunk_with_start_at(next_chunk_start)?
                    {
                        if next_chunk[0] == LF {
                            self.state = BufLineBytesState::End { line_break_len: 2 };
                            // Drop the CR.
                            chunk.pop();
                            (!chunk.is_empty()).then_some(chunk)
                        } else {
                            mid_to_end.chunk = next_chunk.into();
                            Some(chunk)
                        }
                    } else {
                        Some(chunk)
                    }
                } else {
                    Some(chunk)
                }
            }
            BufLineBytesState::End { .. } => None,
        };

        if let Some(chunk) = &chunk {
            assert!(!chunk.is_empty());
            self.next_position += chunk.len();
        }

        Ok(chunk)
    }

    pub fn chunks(&mut self) -> Chunks<'_, 'a, S> {
        Chunks { line_bytes: self }
    }

    #[expect(clippy::missing_errors_doc)]
    pub fn skip_to_end(&mut self) -> io::Result<LineEnd>
    where
        S: io::Read + io::Seek,
    {
        if let BufLineBytesState::End { line_break_len } = self.state {
            Ok(LineEnd {
                position: self.next_position,
                line_break_len,
            })
        } else {
            for _ in self.chunks() {}
            // Again.
            self.skip_to_end()
        }
    }
}

impl<S> Iterator for BufferedLineBytes<'_, S>
where
    S: io::Read + io::Seek,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_byte().transpose()
    }
}

impl<S> Debug for BufferedLineBytes<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferedLineBytes")
            .field("next_position", &self.next_position)
            .field("state", &self.state)
            .finish()
    }
}

pub struct Chunks<'a, 'b, S> {
    line_bytes: &'a mut BufferedLineBytes<'b, S>,
}

impl<S> Iterator for Chunks<'_, '_, S>
where
    S: io::Read + io::Seek,
{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_bytes.next_chunk().transpose()
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

    mod binary_search_by {
        use std::io::Cursor;

        use super::*;

        macro_rules! test {
            ($source_bytes:expr, $buffer_len:expr, $compare:expr, Ok($expected_range:expr) $(,)?) => {
                let result = binary_search_by::<_, io::Error>(
                    Cursor::new($source_bytes),
                    NonZeroUsize::try_from($buffer_len).unwrap(),
                    $compare,
                );
                assert_eq!(result.unwrap(), Ok($expected_range.into()));
            };
        }

        macro_rules! test_by_collecting_bytes {
            ($target:expr, $source_bytes:expr, $buffer_len:expr, $expected_range:expr) => {
                test!(
                    $source_bytes,
                    $buffer_len,
                    move |line_bytes| {
                        let bytes = line_bytes.collect::<io::Result<Vec<_>>>()?;
                        let cmp = bytes.as_slice().cmp(&$target.as_ref());
                        Ok(cmp)
                    },
                    Ok($expected_range),
                );
            };
        }

        macro_rules! test_by_collecting_chunks {
            ($target:expr, $source_bytes:expr, $buffer_len:expr, $expected_range:expr) => {
                test!(
                    $source_bytes,
                    $buffer_len,
                    move |line_bytes| {
                        let mut bytes = Vec::new();
                        for chunk in line_bytes.chunks() {
                            bytes.extend(chunk?);
                        }
                        let cmp = bytes.as_slice().cmp(&$target.as_ref());
                        Ok(cmp)
                    },
                    Ok($expected_range),
                );
            };
        }

        #[test]
        fn cr_without_lf() {
            test_by_collecting_bytes!("a\rb", [b'a', CR, b'b', LF, b'c', b'c'], 2, 0..3);
            test_by_collecting_bytes!("a\rb", [b'a', CR, b'b', LF], 2, 0..3);
            test_by_collecting_bytes!("ab\rc", [b'a', b'b', CR, b'c'], 2, 0..4);
            test_by_collecting_bytes!("bcd\r", [b'a', LF, b'b', b'c', b'd', CR], 2, 2..6);
            test_by_collecting_chunks!("\ra", [CR, b'a', LF, b'b'], 2, 0..2);
            test_by_collecting_chunks!("ab\rc", [b'a', b'b', CR, b'c'], 2, 0..4);
            test_by_collecting_chunks!("a\r", [b'a', CR], 2, 0..2);
        }

        #[test]
        fn additional_case_1() {
            let target = "ab";
            test!(
                [b'a', b'b', LF, b'c'],
                2,
                move |line_bytes| {
                    let mut bytes = Vec::new();
                    if let Some(b) = line_bytes.next() {
                        bytes.push(b?);
                    }
                    for chunk in line_bytes.chunks() {
                        bytes.extend(chunk?);
                    }
                    let cmp = bytes.as_slice().cmp(target.as_ref());
                    Ok(cmp)
                },
                Ok(0..2),
            );
        }
    }
}
