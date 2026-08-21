use core::marker::PhantomData;
use core::ops::Index;

use crate::Indexed;

pub struct View<T, A, F> {
    inner: A,
    f: F,
    phantom: PhantomData<T>,
}

impl<T, A, F> View<T, A, F> {
    pub const fn new(inner: A, f: F) -> Self {
        View {
            inner,
            f,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, A, F, Idx> Index<Idx> for View<&'a T, A, F>
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

impl<'a, T, A, F, Idx> Indexed<'a, Idx> for View<T, A, F>
where
    A: Indexed<'a, Idx>,
    F: Fn(A::Output) -> T,
{
    type Output = T;

    type Indices = A::Indices;

    fn get(&'a self, index: Idx) -> Option<Self::Output> {
        self.inner.get(index).map(&self.f)
    }

    fn indices(&'a self) -> Self::Indices {
        self.inner.indices()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}
