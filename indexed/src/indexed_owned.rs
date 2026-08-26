use crate::{Len, View};

mod as_indexed;

pub use as_indexed::AsIndexed;

/// Provides indexed access to owned elements.
///
/// It can tipically be used when you have a `&[T]` (or `[T; N]`, or `Vec<T>`) and you need to pass
/// a `&[U]` to a function, where `U` is part of `T`, but don't want to allocate a new `Vec<&U>`, or
/// it is cumbersome to have the function receive `&[T]` and `impl FnMut(&T) -> U` parameters.
///
/// Only owned elements can be returned. This includes _owned references_. For instance:
///
/// ```
/// use indexed::{IndexedOwned, Len};
///
/// struct Element<'a, const N: usize> {
///     owned_val: [u32; N],
///     owned_ref: [&'a u32; N],
/// }
///
/// impl<'a, const N: usize> Len for Element<'a, N> {
///     fn len(&self) -> usize {
///         N
///     }
/// }
///
/// // Indexing by `usize` returns owned `u32`.
/// impl<'a, const N: usize> IndexedOwned<usize> for Element<'a, N> {
///     type Output = u32;
///
///     fn get(&self, index: usize) -> Option<Self::Output> {
///         (index < N).then(|| self.owned_val[index])
///     }
/// }
///
/// // Indexing by `char` returns reference `&u32`.
/// impl<'a, const N: usize> IndexedOwned<char> for Element<'a, N> {
///     type Output = &'a u32;
///
///     fn get(&self, index: char) -> Option<Self::Output> {
///         // 'a' is 0, 'b' is 1, etc.
///         let numeric_index = (u32::from(index) - u32::from('a')) as usize;
///         (numeric_index < N).then(|| self.owned_ref[numeric_index])
///     }
/// }
/// ```
///
/// But it cannot return _reference to itself or its components_:
///
/// ```compile_fail
/// use indexed::{IndexedOwned, Len};
///
/// struct Element<const N: usize> {
///     owned_val: [u32; N],
/// }
///
/// impl<const N: usize> Len for Element<N> {
///     fn len(&self) -> usize {
///         N
///     }
/// }
///
/// // Indexing by `usize` returns reference to owned `u32`.
/// impl<const N: usize> IndexedOwned<usize> for Element<N> {
///     type Output = &u32;
///
///     fn get(&self, index: usize) -> Option<Self::Output> {
///         (index < N).then(|| &self.owned_val[index])
///     }
/// }
/// ```
pub trait IndexedOwned<Idx>: Len {
    type Output;

    fn get(&self, index: Idx) -> Option<Self::Output>;

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

    fn as_indexed(&self) -> AsIndexed<&Self>
    where
        Self: Sized,
    {
        AsIndexed::new(self)
    }

    fn into_indexed(self) -> AsIndexed<Self>
    where
        Self: Sized,
    {
        AsIndexed::new(self)
    }
}
