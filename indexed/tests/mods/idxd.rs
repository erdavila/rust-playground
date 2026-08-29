#![expect(clippy::type_complexity)]

use std::ops::{Index, IndexMut, Range};

use indexed::{Indexed, IndexedMut, IndexedRef, Indices, Len, View};

pub(crate) fn new_owned_output_indexed<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> View<T, indexed::indexed_ref::AsIndexed<RefOutputIdxd<Idx, T, N>>, fn(&T) -> T>
where
    Idx: Eq + Copy,
    T: Copy,
{
    new_ref_output_indexed(values).into_view(T::clone)
}

pub(crate) fn new_ref_output_indexed<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> indexed::indexed_ref::AsIndexed<RefOutputIdxd<Idx, T, N>>
where
    Idx: Eq,
{
    new_indexed_ref(values).into_indexed()
}

pub(crate) fn new_indexed_owned<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> indexed::indexed::AsIndexedOwned<
    View<T, indexed::indexed_ref::AsIndexed<RefOutputIdxd<Idx, T, N>>, fn(&T) -> T>,
>
where
    Idx: Eq + Copy,
    T: Copy,
{
    new_owned_output_indexed(values).into_indexed_owned()
}

pub(crate) fn new_indexed_ref<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> RefOutputIdxd<Idx, T, N> {
    RefOutputIdxd(values)
}

pub(crate) fn new_indexed_mut<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> RefOutputIdxd<Idx, T, N> {
    new_indexed_ref(values)
}

pub(crate) struct RefOutputIdxd<Idx, T, const N: usize>(pub(crate) [(Idx, T); N]);

impl<Idx, T, const N: usize> Len for RefOutputIdxd<Idx, T, N> {
    fn len(&self) -> usize {
        N
    }
}

impl<'a, Idx: Copy, T: Copy, const N: usize> Indices<'a, Idx> for RefOutputIdxd<Idx, T, N> {
    type Indices = [Idx; N];

    fn indices(&'a self) -> Self::Indices {
        self.0.map(|(idx, _)| idx)
    }
}

impl<Idx: Eq, T, const N: usize> IndexedRef<Idx> for RefOutputIdxd<Idx, T, N> {
    type Target = T;

    fn get(&self, index: Idx) -> Option<&Self::Target> {
        self.0
            .iter()
            .find_map(|(idx, val)| (*idx == index).then_some(val))
    }
}

impl<Idx: Eq, T, const N: usize> IndexedMut<Idx> for RefOutputIdxd<Idx, T, N> {
    fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Target> {
        self.0
            .iter_mut()
            .find_map(|(idx, val)| (*idx == index).then_some(val))
    }
}

impl<Idx: Eq, T, const N: usize> Index<Idx> for RefOutputIdxd<Idx, T, N> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        self.0
            .iter()
            .find_map(|(idx, val)| (*idx == index).then_some(val))
            .expect("index out of bounds")
    }
}

impl<Idx: Eq, T, const N: usize> IndexMut<Idx> for RefOutputIdxd<Idx, T, N> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0
            .iter_mut()
            .find_map(|(idx, val)| (*idx == index).then_some(val))
            .expect("index out of bounds")
    }
}
