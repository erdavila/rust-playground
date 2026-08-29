use indexed::{IndexedMut, Indices};

use crate::mods::asserts::assert_indexed_mut;
use crate::mods::asserts::entries::{VALUES, values_mapped};
use crate::mods::idxd::new_indexed_mut;
use crate::mods::wrapper::Wrapper;

mod mods;

#[test]
fn get_and_len() {
    let mut idxd_mut = new_indexed_mut(VALUES);

    assert_indexed_mut!(idxd_mut);
}

#[test]
fn view_mut() {
    let mut idxd_mut = new_indexed_mut(values_mapped(Wrapper));

    let mut view = idxd_mut.view_mut(Wrapper::ref_to_ref, Wrapper::mut_to_mut);

    assert_indexed_mut!(view);
}

#[test]
fn into_view_mut() {
    let idxd_mut = new_indexed_mut(values_mapped(Wrapper));

    let mut view = idxd_mut.into_view_mut(Wrapper::ref_to_ref, Wrapper::mut_to_mut);

    assert_indexed_mut!(view);
}

#[test]
fn dyn_compatible() {
    pub(crate) trait IndexedMutWithIndices<Idx>
    where
        Self: IndexedMut<Idx>,
        Self: for<'a> Indices<'a, Idx>,
    {
    }
    impl<A, Idx> IndexedMutWithIndices<Idx> for A
    where
        A: IndexedMut<Idx>,
        A: for<'a> Indices<'a, Idx>,
    {
    }

    let mut idxd_mut = new_indexed_mut(VALUES);

    let mut obj: &mut dyn IndexedMutWithIndices<_, Target = _, Indices = _> = &mut idxd_mut;

    assert_indexed_mut!(obj);
}
