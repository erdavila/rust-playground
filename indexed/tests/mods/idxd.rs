use std::ops::{Index, Range};

use indexed::Indexed;

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

pub(crate) struct RefOutputIdxd<Idx, T, const N: usize>(pub(crate) [(Idx, T); N]);

impl<'a, Idx: Eq + Copy, T: Copy + 'a, const N: usize> Indexed<'a, Idx>
    for RefOutputIdxd<Idx, T, N>
{
    type Output = &'a T;

    type Indices = [Idx; N];

    fn get(&'a self, index: Idx) -> Option<Self::Output> {
        self.0
            .iter()
            .find_map(|(idx, val)| (*idx == index).then_some(val))
    }

    fn indices(&'a self) -> Self::Indices {
        self.0.map(|(idx, _)| idx)
    }

    fn len(&self) -> usize {
        N
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
