use crate::{Len, View};

mod as_indexed;

pub use as_indexed::AsIndexed;

pub trait IndexedRef<Idx>: Len {
    type Target;

    fn get(&self, index: Idx) -> Option<&Self::Target>;

    fn view<T, F>(&self, f: F) -> View<&T, &Self, F>
    where
        F: Fn(&Self::Target) -> &T,
        Self: Sized,
    {
        View::new(self, f)
    }

    fn into_view<'a, T, F>(self, f: F) -> View<&'a T, Self, F>
    where
        F: Fn(&Self::Target) -> &T,
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
