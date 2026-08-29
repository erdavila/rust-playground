use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

use crate::{IndexedMut, IndexedRef, Indices, Len};

pub struct ViewMut<T, A, F, FMut> {
    inner: A,
    f: F,
    f_mut: FMut,
    phantom: PhantomData<T>,
}

impl<T, A, F, FMut> ViewMut<&T, A, F, FMut> {
    // `U` should be the same as `A::Target`, but we are not constraining `A` here.
    pub const fn new<U>(inner: A, f: F, f_mut: FMut) -> Self
    where
        F: Fn(&U) -> &T,
        FMut: FnMut(&mut U) -> &mut T,
    {
        ViewMut {
            inner,
            f,
            f_mut,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, A, F, FMut, Idx> Index<Idx> for ViewMut<&'a T, A, F, FMut>
where
    A: Index<Idx>,
    A::Output: 'a,
    F: Fn(&A::Output) -> &T,
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        (self.f)(&self.inner[index])
    }
}

impl<'a, T, A, F, FMut, Idx> IndexMut<Idx> for ViewMut<&'a T, A, F, FMut>
where
    A: IndexMut<Idx>,
    A::Output: 'a,
    F: Fn(&A::Output) -> &T,
    FMut: FnMut(&mut A::Output) -> &mut T,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        (self.f_mut)(&mut self.inner[index])
    }
}

impl<T, A, F, FMut> Len for ViewMut<T, A, F, FMut>
where
    A: Len,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, T, A, F, FMut, Idx> Indices<'a, Idx> for ViewMut<T, A, F, FMut>
where
    A: Indices<'a, Idx>,
{
    type Indices = A::Indices;

    fn indices(&'a self) -> Self::Indices {
        self.inner.indices()
    }
}

impl<'a, T, A, F, FMut, Idx> IndexedRef<Idx> for ViewMut<&'a T, A, F, FMut>
where
    A: IndexedRef<Idx>,
    A::Target: 'a,
    F: Fn(&A::Target) -> &T,
{
    type Target = T;

    fn get(&self, index: Idx) -> Option<&Self::Target> {
        self.inner.get(index).map(&self.f)
    }
}

impl<'a, T, A, F, FMut, Idx> IndexedMut<Idx> for ViewMut<&'a T, A, F, FMut>
where
    A: IndexedMut<Idx>,
    A::Target: 'a,
    F: Fn(&A::Target) -> &T,
    FMut: FnMut(&mut A::Target) -> &mut T,
{
    fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Target> {
        self.inner.get_mut(index).map(&mut self.f_mut)
    }
}
