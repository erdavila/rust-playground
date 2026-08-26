use crate::{Indexed, IndexedOwned, Indices, Len};

/// Adapts an [`Indexed`] into an [`IndexedOwned`].
pub struct AsIndexedOwned<A> {
    inner: A,
}

impl<A> AsIndexedOwned<A> {
    pub fn new(inner: A) -> Self {
        AsIndexedOwned { inner }
    }
}

impl<A> Len for AsIndexedOwned<A>
where
    A: Len,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, A, Idx> Indices<'a, Idx> for AsIndexedOwned<A>
where
    A: Indices<'a, Idx>,
{
    type Indices = A::Indices;

    fn indices(&'a self) -> Self::Indices {
        self.inner.indices()
    }
}

impl<T, A, Idx> IndexedOwned<Idx> for AsIndexedOwned<A>
where
    A: for<'a> Indexed<'a, Idx, Output = T>,
{
    type Output = T;

    fn get(&self, index: Idx) -> Option<Self::Output> {
        self.inner.get(index)
    }
}
