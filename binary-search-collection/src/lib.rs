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
