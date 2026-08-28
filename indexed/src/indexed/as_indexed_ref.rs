use core::ops::Index;

use crate::{Indexed, IndexedRef, Indices, Len};

/// Adapts an [`Indexed`] into an [`IndexedRef`].
pub struct AsIndexedRef<A> {
    inner: A,
}

impl<A> AsIndexedRef<A> {
    pub fn new(inner: A) -> Self {
        AsIndexedRef { inner }
    }
}

impl<A> Len for AsIndexedRef<A>
where
    A: Len,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, A, Idx> Indices<'a, Idx> for AsIndexedRef<A>
where
    A: Indices<'a, Idx>,
{
    type Indices = A::Indices;

    fn indices(&'a self) -> Self::Indices {
        self.inner.indices()
    }
}

impl<T, A, Idx> IndexedRef<Idx> for AsIndexedRef<A>
where
    A: for<'a> Indexed<'a, Idx, Output = &'a T>,
{
    type Target = T;

    fn get(&self, index: Idx) -> Option<&Self::Target> {
        self.inner.get(index)
    }
}

impl<A, Idx> Index<Idx> for AsIndexedRef<A>
where
    A: Index<Idx>,
{
    type Output = A::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.inner[index]
    }
}
