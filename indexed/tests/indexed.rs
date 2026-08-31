use indexed::{Indexed, Indices};

use crate::mods::asserts::entries::{NONE_INDEX, VALUES, expected, values_mapped};
use crate::mods::asserts::{assert_indexed, assert_indexed_owned};
use crate::mods::idxd::{new_owned_output_indexed, new_ref_output_indexed};
use crate::mods::wrapper::Wrapper;

mod mods;

mod owned_output {
    use super::*;

    #[test]
    fn get_and_len() {
        let idxd = new_owned_output_indexed(VALUES);

        assert_indexed!(owned: idxd);
    }

    #[test]
    fn view() {
        let idxd = new_owned_output_indexed(values_mapped(Wrapper));

        let view = idxd.view(Wrapper::owned_to_owned);

        assert_indexed!(owned: view);
    }

    #[test]
    fn into_view() {
        let idxd = new_owned_output_indexed(values_mapped(Wrapper));

        let view = idxd.into_view(Wrapper::owned_to_owned);

        assert_indexed!(owned: view);
    }

    #[test]
    fn as_indexed_owned() {
        let idxd = new_owned_output_indexed(VALUES);

        let idxd_owned = idxd.as_indexed_owned();

        assert_indexed_owned!(idxd_owned);
    }

    #[test]
    fn into_indexed_owned() {
        let idxd = new_owned_output_indexed(VALUES);

        let idxd_owned = idxd.into_indexed_owned();

        assert_indexed_owned!(idxd_owned);
    }

    #[test]
    fn as_indexed_ref() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/conversion/owned_output_indexed/as_indexed_ref.rs");
    }

    #[test]
    fn into_indexed_ref() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/conversion/owned_output_indexed/into_indexed_ref.rs");
    }

    #[test]
    fn as_fn() {
        let idxd = new_owned_output_indexed(VALUES);

        let fn_ = idxd.as_fn();

        for (k, v) in expected::as_owned_owned() {
            assert_eq!(fn_(k), Some(v));
        }
        assert_eq!(fn_(NONE_INDEX), None);
    }

    #[test]
    fn dyn_compatible() {
        pub(crate) trait IndexedWithIndicesOwned<'a, Idx>
        where
            Self: Indexed<'a, Idx>,
            Self: Indices<'a, Idx>,
        {
        }
        impl<'a, A, Idx> IndexedWithIndicesOwned<'a, Idx> for A
        where
            A: Indexed<'a, Idx>,
            A: Indices<'a, Idx>,
        {
        }

        let idxd = new_owned_output_indexed(VALUES);

        let obj: &dyn for<'a> IndexedWithIndicesOwned<'a, _, Output = _, Indices = _> = &idxd;

        assert_indexed!(owned: obj);
    }
}

mod ref_output {
    use super::*;
    use crate::mods::asserts::{assert_index, assert_indexed_ref};

    #[test]
    fn get_and_len() {
        let idxd = new_ref_output_indexed(VALUES);

        assert_indexed!(ref: idxd);
    }

    #[test]
    fn view() {
        let idxd = new_ref_output_indexed(values_mapped(Wrapper));

        let view = idxd.view(Wrapper::ref_to_ref);

        assert_indexed!(ref: view);
    }

    #[test]
    fn into_view() {
        let idxd = new_ref_output_indexed(values_mapped(Wrapper));

        let view = idxd.into_view(Wrapper::ref_to_ref);

        assert_indexed!(ref: view);
    }

    #[test]
    fn as_indexed_owned() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/conversion/ref_output_indexed/as_indexed_owned.rs");
    }

    #[test]
    fn into_indexed_owned() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/conversion/ref_output_indexed/into_indexed_owned.rs");
    }

    #[test]
    fn as_indexed_ref() {
        let idxd = new_ref_output_indexed(VALUES);

        let idxd_ref = idxd.as_indexed_ref();

        assert_indexed_ref!(idxd_ref);
    }

    #[test]
    fn into_indexed_ref() {
        let idxd = new_ref_output_indexed(VALUES);

        let idxd_ref = idxd.into_indexed_ref();

        assert_indexed_ref!(idxd_ref);
        // `Index::index` is available when the inner is owned and the output is a reference.
        assert_index!(idxd_ref);
    }

    #[test]
    fn as_fn() {
        let idxd = new_ref_output_indexed(VALUES);

        let fn_ = idxd.as_fn();

        for (k, v) in expected::as_owned_ref() {
            assert_eq!(fn_(k), Some(v));
        }
        assert_eq!(fn_(NONE_INDEX), None);
    }

    #[test]
    fn dyn_compatible() {
        pub(crate) trait IndexedWithIndicesRef<Idx, Target: ?Sized>
        where
            Self: for<'a> Indexed<'a, Idx, Output = &'a Target>,
            Self: for<'a> Indices<'a, Idx>,
        {
        }
        impl<A, Idx, Target: ?Sized> IndexedWithIndicesRef<Idx, Target> for A
        where
            A: for<'a> Indexed<'a, Idx, Output = &'a Target>,
            A: for<'a> Indices<'a, Idx> + ?Sized,
        {
        }

        let idxd = new_ref_output_indexed(VALUES);

        let obj: &dyn IndexedWithIndicesRef<_, _, Indices = _> = &idxd;

        assert_indexed!(ref: obj);
    }
}

#[test]
fn output_requirement() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Output {
        Owned,
        Reference,
    }
    impl Output {
        fn path_component(self) -> &'static str {
            match self {
                Output::Owned => "owned",
                Output::Reference => "ref",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Dispatch {
        Static,
        Dynamic,
    }
    impl Dispatch {
        fn path_component(&self) -> &str {
            match self {
                Dispatch::Static => "static",
                Dispatch::Dynamic => "dynamic",
            }
        }
    }

    let t = trybuild::TestCases::new();
    let base_path = "tests/indexed_output_requirement";

    let mut count = 0;
    let max = std::env::args()
        .skip(1)
        .find_map(|arg| arg.parse::<u32>().ok())
        .unwrap_or(12);
    for requirement in [Some(Output::Owned), Some(Output::Reference), None] {
        for dispatch in [Dispatch::Static, Dispatch::Dynamic] {
            for idxd_output in [Output::Owned, Output::Reference] {
                if count >= max {
                    return;
                }
                count += 1;

                let path = format!(
                    "{base_path}/{}/require_{}/{}_output_idxd.rs",
                    dispatch.path_component(),
                    requirement.map_or("any", |output| output.path_component()),
                    idxd_output.path_component(),
                );

                let should_pass = requirement.is_none_or(|req| req == idxd_output);

                if should_pass {
                    t.pass(path);
                } else {
                    t.compile_fail(path);
                }
            }
        }
    }
}
