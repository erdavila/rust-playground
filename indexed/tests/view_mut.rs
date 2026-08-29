use indexed::ViewMut;

use crate::mods::asserts::entries::values_mapped;
use crate::mods::asserts::{
    assert_index, assert_index_mut, assert_indexed_mut, assert_indexed_ref,
};
use crate::mods::idxd::{new_indexed_mut, new_indexed_ref};
use crate::mods::wrapper::Wrapper;

mod mods;

#[test]
fn indexed_ref_inner() {
    let idxd_ref = new_indexed_ref(values_mapped(Wrapper));

    let ref_inner = &idxd_ref;
    let view = ViewMut::new(ref_inner, Wrapper::ref_to_ref, Wrapper::mut_to_mut);
    assert_indexed_ref!(view);

    let owned_inner = idxd_ref;
    let view = ViewMut::new(owned_inner, Wrapper::ref_to_ref, Wrapper::mut_to_mut);
    assert_indexed_ref!(view);
    // `Index::index` is available when the inner is owned and the output is a reference.
    assert_index!(view);
}

#[test]
fn indexed_mut_inner() {
    let mut idxd_mut = new_indexed_mut(values_mapped(Wrapper));

    let ref_inner = &mut idxd_mut;
    let mut view = ViewMut::new(ref_inner, Wrapper::ref_to_ref, Wrapper::mut_to_mut);
    assert_indexed_mut!(view);

    let owned_inner = idxd_mut;
    let mut view = ViewMut::new(owned_inner, Wrapper::ref_to_ref, Wrapper::mut_to_mut);
    assert_indexed_mut!(view);
    // `Index_mut::index_mut` is available when the inner is owned and the output is a reference.
    assert_index_mut!(view);
}
