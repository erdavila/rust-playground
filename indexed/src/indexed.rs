use crate::{Len, View};

mod as_indexed_owned;

pub use as_indexed_owned::AsIndexedOwned;

/// Provides indexed access to elements.
///
/// It can tipically be used when you have a `&[T]` (or `[T; N]`, or `Vec<T>`) and you need to pass
/// a `&[U]` to a function, where `U` is part of `T`, but don't want to allocate a new `Vec<&U>`, or
/// it is cumbersome to have the function receive `&[T]` and `impl FnMut(&T) -> U` parameters.
///
/// The `Output` can be either an owned value, or a reference.
pub trait Indexed<'a, Idx>: Len {
    type Output;

    fn get(&'a self, index: Idx) -> Option<Self::Output>;

    fn view<T, F>(&self, f: F) -> View<T, &Self, F>
    where
        F: Fn(Self::Output) -> T,
        Self: Sized,
    {
        View::new(self, f)
    }

    fn into_view<T, F>(self, f: F) -> View<T, Self, F>
    where
        F: Fn(Self::Output) -> T,
        Self: Sized,
    {
        View::new(self, f)
    }

    fn as_indexed_owned(&self) -> AsIndexedOwned<&Self>
    where
        Self: Sized,
    {
        AsIndexedOwned::new(self)
    }

    fn into_indexed_owned(self) -> AsIndexedOwned<Self>
    where
        Self: Sized,
    {
        AsIndexedOwned::new(self)
    }
}
