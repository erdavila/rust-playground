#![expect(unused)]

#[path = "../mods/mod.rs"]
mod mods;

use indexed::Indexed;

pub(crate) use mods::asserts::entries::VALUES;
pub(crate) use mods::idxd::{new_owned_output_indexed, new_ref_output_indexed};

pub(crate) mod owned_output {
    use super::*;

    macro_rules! idxd {
        () => {
            new_owned_output_indexed(VALUES)
        };
    }
    pub(crate) use idxd;

    pub(crate) fn required_by_static_dispatch<Idx, T>(_: &impl for<'a> Indexed<'a, Idx, Output = T>) {}

    pub(crate) fn required_by_dynamic_dispatch<Idx, T, Idxs>(_: &dyn for<'a> Indexed<'a, Idx, Output = T, Indices = Idxs>) {}
}

pub(crate) mod ref_output {
    use super::*;

    macro_rules! idxd {
        () => {
            new_ref_output_indexed(VALUES)
        };
    }
    pub(crate) use idxd;

    pub(crate) fn required_by_static_dispatch<Idx, T>(_: &impl for<'a> Indexed<'a, Idx, Output = &'a T>) {}

    pub(crate) fn required_by_dynamic_dispatch<Idx, T, Idxs>(_: &dyn for<'a> Indexed<'a, Idx, Output = &'a T, Indices = Idxs>) {}
}

pub(crate) mod any_output {
    use super::*;

    pub(crate) fn required_by_static_dispatch<Idx>(_: &impl for<'a> Indexed<'a, Idx>) {}

    pub(crate) fn required_by_dynamic_dispatch<A>(_: &A) {
        // Impossible. Dynamic dispatch must choose between owned or reference `Output`.
    }
}
