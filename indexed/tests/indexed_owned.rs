use indexed::{IndexedOwned, Indices};

use crate::mods::asserts::entries::{NONE_INDEX, VALUES, expected, values_mapped};
use crate::mods::asserts::{assert_indexed, assert_indexed_owned};
use crate::mods::idxd::new_indexed_owned;
use crate::mods::wrapper::Wrapper;

mod mods;

#[test]
fn get_and_len() {
    let idxd_owned = new_indexed_owned(VALUES);

    assert_indexed_owned!(idxd_owned);
}

#[test]
fn view() {
    let idxd_owned = new_indexed_owned(values_mapped(Wrapper));

    let view = idxd_owned.view(Wrapper::owned_to_owned);

    assert_indexed_owned!(view);
}

#[test]
fn into_view() {
    let idxd_owned = new_indexed_owned(values_mapped(Wrapper));

    let view = idxd_owned.into_view(Wrapper::owned_to_owned);

    assert_indexed_owned!(view);
}

#[test]
fn as_indexed() {
    let idxd_owned = new_indexed_owned(VALUES);

    let idxd = idxd_owned.as_indexed();

    assert_indexed!(owned: idxd);
}

#[test]
fn into_indexed() {
    let idxd_owned = new_indexed_owned(VALUES);

    let idxd = idxd_owned.into_indexed();

    assert_indexed!(owned: idxd);
}

#[test]
fn as_fn() {
    let idxd = new_indexed_owned(VALUES);

    let fn_ = idxd.as_fn();

    for (k, v) in expected::as_owned_owned() {
        assert_eq!(fn_(k), Some(v));
    }
    assert_eq!(fn_(NONE_INDEX), None);
}

#[test]
fn into_fn() {
    let idxd = new_indexed_owned(VALUES);

    let fn_ = idxd.into_fn();

    for (k, v) in expected::as_owned_owned() {
        assert_eq!(fn_(k), Some(v));
    }
    assert_eq!(fn_(NONE_INDEX), None);
}

#[test]
fn dyn_compatible() {
    pub(crate) trait IndexedOwnedWithIndices<Idx>
    where
        Self: IndexedOwned<Idx>,
        Self: for<'a> Indices<'a, Idx>,
    {
    }
    impl<A, Idx> IndexedOwnedWithIndices<Idx> for A
    where
        A: IndexedOwned<Idx>,
        A: for<'a> Indices<'a, Idx>,
    {
    }

    let idxd_owned = new_indexed_owned(VALUES);

    let obj: &dyn IndexedOwnedWithIndices<_, Output = _, Indices = _> = &idxd_owned;

    assert_indexed_owned!(obj);
}
