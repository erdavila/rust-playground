use indexed::{IndexedRef, Indices};

use crate::mods::asserts::entries::{NONE_INDEX, VALUES, expected, values_mapped};
use crate::mods::asserts::{assert_index, assert_indexed, assert_indexed_ref};
use crate::mods::idxd::new_indexed_ref;
use crate::mods::wrapper::Wrapper;

mod mods;

#[test]
fn get_and_len() {
    let idxd_ref = new_indexed_ref(VALUES);

    assert_indexed_ref!(idxd_ref);
}

#[test]
fn view() {
    let idxd_ref = new_indexed_ref(values_mapped(Wrapper));

    let view = idxd_ref.view(Wrapper::ref_to_ref);

    assert_indexed_ref!(view);
}

#[test]
fn into_view() {
    let idxd_ref = new_indexed_ref(values_mapped(Wrapper));

    let view = idxd_ref.into_view(Wrapper::ref_to_ref);

    assert_indexed_ref!(view);
}

#[test]
fn as_indexed() {
    let idxd_ref = new_indexed_ref(VALUES);

    let idxd = idxd_ref.as_indexed();

    assert_indexed!(ref: idxd);
}

#[test]
fn into_indexed() {
    let idxd_ref = new_indexed_ref(VALUES);

    let idxd = idxd_ref.into_indexed();

    assert_indexed!(ref: idxd);
    // `Index::index` is available when the inner is owned and the output is a reference.
    assert_index!(idxd);
}

#[test]
fn as_fn() {
    let idxd = new_indexed_ref(VALUES);

    let fn_ = idxd.as_fn();

    for (k, v) in expected::as_owned_ref() {
        assert_eq!(fn_(k), Some(v));
    }
    assert_eq!(fn_(NONE_INDEX), None);
}

#[test]
fn dyn_compatible() {
    pub(crate) trait IndexedRefWithIndices<Idx>
    where
        Self: IndexedRef<Idx>,
        Self: for<'a> Indices<'a, Idx>,
    {
    }
    impl<A, Idx> IndexedRefWithIndices<Idx> for A
    where
        A: IndexedRef<Idx>,
        A: for<'a> Indices<'a, Idx>,
    {
    }

    let idxd_ref = new_indexed_ref(VALUES);

    let obj: &dyn IndexedRefWithIndices<_, Target = _, Indices = _> = &idxd_ref;

    assert_indexed_ref!(obj);
}
