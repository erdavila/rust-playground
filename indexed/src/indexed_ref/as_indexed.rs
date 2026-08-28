use core::ops::Index;

use crate::{Indexed, IndexedRef, Indices, Len};

/// Adapts an [`IndexedRef`] into an [`Indexed`].
pub struct AsIndexed<A> {
    inner: A,
}

impl<A> AsIndexed<A> {
    pub fn new(inner: A) -> Self {
        AsIndexed { inner }
    }
}

impl<A> Len for AsIndexed<A>
where
    A: Len,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, A, Idx> Indices<'a, Idx> for AsIndexed<A>
where
    A: Indices<'a, Idx>,
{
    type Indices = A::Indices;

    fn indices(&'a self) -> Self::Indices {
        self.inner.indices()
    }
}

impl<'a, A, Idx> Indexed<'a, Idx> for AsIndexed<A>
where
    A: IndexedRef<Idx>,
    A::Target: 'a,
{
    type Output = &'a A::Target;

    fn get(&'a self, index: Idx) -> Option<Self::Output> {
        self.inner.get(index)
    }
}

impl<A, Idx> Index<Idx> for AsIndexed<A>
where
    A: Index<Idx>,
{
    type Output = A::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.inner[index]
    }
}
