use crate::View;

/// Provides indexed access to elements.
///
/// It can tipically be used when you have a `&[T]` (or `[T; N]`, or `Vec<T>`) and you need to pass
/// a `&[U]` to a function, where `U` is part of `T`, but don't want to allocate a new `Vec<&U>`, or
/// it is cumbersome to have the function receive `&[T]` and `impl FnMut(&T) -> U` parameters.
///
/// The `Output` can be either an owned value, or a reference.
pub trait Indexed<'a, Idx> {
    type Output;

    type Indices: IntoIterator<Item = Idx>;

    fn get(&'a self, index: Idx) -> Option<Self::Output>;

    fn indices(&'a self) -> Self::Indices;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
}
