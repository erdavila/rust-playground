#![warn(missing_docs)]
#![doc(test(attr(deny(warnings))))] // Warnings in doctests will be errors instead of silenced

//! Provides heterogeneous lists.

mod core;
mod get;
mod into_tuple;
mod macros;
pub mod tuples;

pub use core::*;
pub use get::*;
pub use into_tuple::*;
