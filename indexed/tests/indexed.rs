use indexed::Indexed;

use crate::mods::asserts::assert_indexed;
use crate::mods::asserts::entries::{VALUES, values_mapped};
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
    fn dyn_compatible() {
        let idxd = new_owned_output_indexed(VALUES);

        let obj: &dyn for<'a> Indexed<'a, _, Output = _, Indices = _> = &idxd;

        assert_indexed!(owned: obj);
    }
}

mod ref_output {
    use super::*;

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
    fn dyn_compatible() {
        let idxd = new_ref_output_indexed(VALUES);

        let obj: &dyn for<'a> Indexed<'a, _, Output = &'a _, Indices = _> = &idxd;

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
