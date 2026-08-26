#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;
pub mod indexed;
pub mod indexed_owned;
mod indices;
mod len;
mod view;

#[doc(inline)]
pub use indexed::Indexed;
#[doc(inline)]
pub use indexed_owned::IndexedOwned;
pub use indices::*;
pub use len::*;
pub use view::*;
