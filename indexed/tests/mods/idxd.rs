#![expect(clippy::type_complexity)]

use std::ops::{Index, Range};

use indexed::{Indexed, Indices, Len};

pub(crate) fn new_owned_output_indexed<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> indexed::View<T, RefOutputIdxd<Idx, T, N>, impl Fn(&T) -> T>
where
    Idx: Eq + Copy,
    T: Copy,
{
    new_ref_output_indexed(values).into_view(T::clone)
}

pub(crate) fn new_ref_output_indexed<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> RefOutputIdxd<Idx, T, N> {
    RefOutputIdxd(values)
}

pub(crate) fn new_indexed_owned<Idx, T, const N: usize>(
    values: [(Idx, T); N],
) -> indexed::indexed::AsIndexedOwned<indexed::View<T, RefOutputIdxd<Idx, T, N>, impl Fn(&T) -> T>>
where
    Idx: Eq + Copy,
    T: Copy,
{
    new_owned_output_indexed(values).into_indexed_owned()
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

impl<'a, Idx: Eq, T: 'a, const N: usize> Indexed<'a, Idx> for RefOutputIdxd<Idx, T, N> {
    type Output = &'a T;

    fn get(&'a self, index: Idx) -> Option<Self::Output> {
        self.0
            .iter()
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
