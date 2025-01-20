#![warn(missing_docs)]
#![doc(test(attr(deny(warnings))))] // Warnings in doctests will be errors instead of silenced

//! Provides heterogeneous lists.

mod core;
mod for_each;
mod get;
mod into_tuple;
mod macros;
mod map;
pub mod tuples;

pub use core::*;
pub use for_each::{ForEach, Over as ForEachOver};
pub use get::*;
pub use into_tuple::*;
pub use map::{Map, Over as MapOver};
