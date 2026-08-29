use crate::{IndexedRef, ViewMut};

pub trait IndexedMut<Idx>: IndexedRef<Idx> {
    fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Target>;

    fn view_mut<T, F, FMut>(&mut self, f: F, f_mut: FMut) -> ViewMut<&T, &mut Self, F, FMut>
    where
        F: Fn(&Self::Target) -> &T,
        FMut: FnMut(&mut Self::Target) -> &mut T,
        Self: Sized,
    {
        ViewMut::new(self, f, f_mut)
    }

    fn into_view_mut<'a, T, F, FMut>(self, f: F, f_mut: FMut) -> ViewMut<&'a T, Self, F, FMut>
    where
        F: Fn(&Self::Target) -> &T,
        FMut: FnMut(&mut Self::Target) -> &mut T,
        Self: Sized,
    {
        ViewMut::new(self, f, f_mut)
    }
}
