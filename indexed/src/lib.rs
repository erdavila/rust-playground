#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod impls;
pub mod indexed;
pub mod indexed_mut;
pub mod indexed_owned;
pub mod indexed_ref;
mod indices;
mod len;
mod view;
mod view_mut;

#[doc(inline)]
pub use indexed::Indexed;
#[doc(inline)]
pub use indexed_mut::IndexedMut;
#[doc(inline)]
pub use indexed_owned::IndexedOwned;
#[doc(inline)]
pub use indexed_ref::IndexedRef;
pub use indices::*;
pub use len::*;
pub use view::*;
pub use view_mut::*;
