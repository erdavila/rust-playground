#![no_std]

//! Provides binary search functions for ranges where distinct positions may not correspond to distinct values.
//!
//! This includes cases where some positions are ignored, or where values can span across multiple positions.
//!
//! It is important to define the following value types:
//!
//! - _elements_: The low-level values. Indexes and ranges refer to them.
//! - _items_: The values that are compared during the search. They are somehow derived from the _elements_.
//!
//! The variants include:
//!
//! - searching in slices, or in an unspecified source, where the _elements_ are fetched on-demand;
//! - searching for subslices, or for values of an arbitrary type;
//! - searching specifically for lines as from a text file.

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
