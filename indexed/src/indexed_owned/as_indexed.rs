use crate::{Indexed, IndexedOwned, Indices, Len};

/// Adapts an [`IndexedOwned`] into an [`Indexed`].
pub struct AsIndexed<A> {
    inner: A,
}

impl<A> AsIndexed<A> {
    pub const fn new(inner: A) -> Self {
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

impl<A, Idx> Indexed<'_, Idx> for AsIndexed<A>
where
    A: IndexedOwned<Idx>,
{
    type Output = A::Output;

    fn get(&'_ self, index: Idx) -> Option<Self::Output> {
        self.inner.get(index)
    }
}
