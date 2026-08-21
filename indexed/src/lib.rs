#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;
mod indexed;
mod view;

pub use indexed::*;
pub use view::*;
