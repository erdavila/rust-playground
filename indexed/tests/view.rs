use indexed::View;

use crate::mods::asserts::entries::{expected, values_mapped};
use crate::mods::asserts::{assert_indexed, assert_indexed_owned};
use crate::mods::idxd::{new_indexed_owned, new_owned_output_indexed, new_ref_output_indexed};
use crate::mods::wrapper::Wrapper;

mod mods;

mod indexed_inner {
    use super::*;

    #[test]
    fn owned_output() {
        let idxd = new_owned_output_indexed(values_mapped(Wrapper));

        let ref_inner = &idxd;
        let view = View::new(ref_inner, Wrapper::owned_to_owned);
        assert_indexed!(owned: view);

        let owned_inner = idxd;
        let view = View::new(owned_inner, Wrapper::owned_to_owned);
        assert_indexed!(owned: view);
    }

    mod ref_output {
        use super::*;

        #[test]
        fn to_owned_output() {
            let idxd = new_ref_output_indexed(values_mapped(Wrapper));

            let ref_inner = &idxd;
            let view = View::new(ref_inner, Wrapper::ref_to_owned);
            assert_indexed!(owned: view);

            let owned_inner = idxd;
            let view = View::new(owned_inner, Wrapper::ref_to_owned);
            assert_indexed!(owned: view);
        }

        #[test]
        fn to_ref_output() {
            let idxd = new_ref_output_indexed(values_mapped(Wrapper));

            let ref_inner = &idxd;
            let view = View::new(ref_inner, Wrapper::ref_to_ref);
            assert_indexed!(ref: view);

            let owned_inner = idxd;
            let view = View::new(owned_inner, Wrapper::ref_to_ref);
            assert_indexed!(ref: view);
            // `Index::index` is available when the inner is owned and the output is a reference.
            for (idx, val) in expected::as_owned_owned() {
                assert_eq!(view[idx], val);
            }
        }
    }
}

#[test]
fn indexed_owned_inner() {
    let idxd_owned = new_indexed_owned(values_mapped(Wrapper));

    let ref_inner = &idxd_owned;
    let view = View::new(ref_inner, Wrapper::owned_to_owned);
    assert_indexed_owned!(view);

    let owned_inner = idxd_owned;
    let view = View::new(owned_inner, Wrapper::owned_to_owned);
    assert_indexed_owned!(view);
}
