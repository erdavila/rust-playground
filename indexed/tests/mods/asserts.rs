macro_rules! assert_indexed {
    (owned: $idxd:expr) => {{
        use $crate::mods::asserts::entries::{expected, NONE_INDEX};

        $crate::mods::asserts::assert_indexed!($idxd, expected::as_owned_owned(), NONE_INDEX);
    }};
    (ref: $idxd:expr) => {{
        use $crate::mods::asserts::entries::{expected, NONE_INDEX};

        $crate::mods::asserts::assert_indexed!($idxd, expected::as_owned_ref(), NONE_INDEX);
    }};
    ($idxd:expr, $expected:expr, $none_index:expr $(,)?) => {
        $crate::mods::asserts::assert_indexed!(@ $idxd, $expected, $none_index);
        $crate::mods::asserts::assert_indexed!(@ &$idxd, $expected, $none_index);
    };
    (@ $idxd:expr, $expected:expr, $none_index:expr) => {{
        use indexed::Indexed;

        $crate::mods::asserts::__assert_indexed!(Indexed; $idxd, $expected, $none_index);
    }};
}
pub(crate) use assert_indexed;

macro_rules! assert_indexed_owned {
    ($idxd_owned:expr) => {{
        use $crate::mods::asserts::entries::{expected, NONE_INDEX};

        $crate::mods::asserts::assert_indexed_owned!($idxd_owned, expected::as_owned_owned(), NONE_INDEX);
    }};
    ($idxd_owned:expr, $expected:expr, $none_index:expr $(,)?) => {
        $crate::mods::asserts::assert_indexed_owned!(@ $idxd_owned, $expected, $none_index);
        $crate::mods::asserts::assert_indexed_owned!(@ &$idxd_owned, $expected, $none_index);
    };
    (@ $idxd_owned:expr, $expected:expr, $none_index:expr) => {{
        use indexed::IndexedOwned;

        $crate::mods::asserts::__assert_indexed!(IndexedOwned; $idxd_owned, $expected, $none_index);
    }};
}
pub(crate) use assert_indexed_owned;

macro_rules! assert_indexed_ref {
    ($idxd_ref:expr) => {{
        use $crate::mods::asserts::entries::{expected, NONE_INDEX};

        $crate::mods::asserts::assert_indexed_ref!($idxd_ref, expected::as_owned_ref(), NONE_INDEX);
    }};
    ($idxd_ref:expr, $expected:expr, $none_index:expr $(,)?) => {
        $crate::mods::asserts::assert_indexed_ref!(@ $idxd_ref, $expected, $none_index);
        $crate::mods::asserts::assert_indexed_ref!(@ &$idxd_ref, $expected, $none_index);
    };
    (@ $idxd_ref:expr, $expected:expr, $none_index:expr) => {{
        use indexed::IndexedRef;

        $crate::mods::asserts::__assert_indexed!(IndexedRef; $idxd_ref, $expected, $none_index);
    }};
}
pub(crate) use assert_indexed_ref;

macro_rules! assert_indexed_mut {
    ($idxd_mut:expr) => {{
        use $crate::mods::asserts::entries::{expected, NONE_INDEX};

        $crate::mods::asserts::assert_indexed_mut!($idxd_mut, expected::as_owned_mut(), NONE_INDEX);
    }};
    ($idxd_mut:expr, $expected:expr, $none_index:expr $(,)?) => {
        $crate::mods::asserts::assert_indexed_mut!(@ $idxd_mut, $expected, $none_index);
        $crate::mods::asserts::assert_indexed_mut!(@ &mut $idxd_mut, $expected, $none_index);
    };
    (@ $idxd_mut:expr, $expected:expr, $none_index:expr) => {{
        use indexed::IndexedMut;

        $crate::mods::asserts::__assert_indexed!(IndexedMut, mut; $idxd_mut, $expected, $none_index);
        $crate::mods::asserts::assert_indexed_ref!($idxd_mut, $expected.map(|(k, v)| (k, &*v)), $none_index);
    }};
}
pub(crate) use assert_indexed_mut;

macro_rules! __assert_indexed {
    ($trait:ident $(, $mut:tt)?; $a:expr, $expected:expr, $none_index:expr) => {{
        use std::collections::{BTreeMap, BTreeSet};
        use indexed::{Indices, Len};
        use $crate::mods::asserts::__call_get_method;

        let expected: BTreeMap<_, _> = $expected.into_iter().collect();

        assert_eq!(Len::len(&$a), expected.len());

        assert_eq!(
            Indices::indices(&$a).into_iter().collect::<BTreeSet<_>>(),
            expected.keys().copied().collect::<BTreeSet<_>>(),
        );

        for (k, v) in expected {
            assert_eq!(__call_get_method!($trait $(, $mut)?; $a, k), Some(v));
        }
        assert_eq!(__call_get_method!($trait $(, $mut)?; $a, $none_index), None);
    }};
}
pub(crate) use __assert_indexed;

macro_rules! __call_get_method {
    ($trait:ident, mut; $a:expr, $index:expr) => {
        $trait::get_mut(&mut $a, $index)
    };
    ($trait:ident; $a:expr, $index:expr) => {
        $trait::get(&$a, $index)
    };
}
pub(crate) use __call_get_method;

macro_rules! assert_index {
    ($idxd:ident) => {
        for (idx, val) in $crate::mods::asserts::entries::expected::as_owned_ref() {
            assert_eq!(&$idxd[idx], val);
        }
    };
}
pub(crate) use assert_index;

macro_rules! assert_index_mut {
    ($idxd:ident) => {
        for (idx, val) in $crate::mods::asserts::entries::expected::as_owned_mut() {
            assert_eq!(&mut $idxd[idx], val);
        }
    };
}
pub(crate) use assert_index_mut;

pub(crate) mod entries {
    use std::array;

    pub(crate) const VALUES: [(char, u32); 3] = [('a', 1), ('b', 2), ('c', 3)];
    pub(crate) const NONE_INDEX: char = 'd';

    pub(crate) fn values_mapped<T>(f: fn(u32) -> T) -> [(char, T); 3] {
        array::from_fn(|i| {
            let (idx, val) = VALUES[i];
            (idx, f(val))
        })
    }

    pub(crate) mod expected {
        static mut EXPECTED: [(char, u32); 3] = super::VALUES;

        pub(crate) fn as_owned_owned() -> impl Iterator<Item = (char, u32)> {
            expected().iter().map(|(idx, val)| (*idx, *val))
        }

        pub(crate) fn as_owned_ref() -> impl Iterator<Item = (char, &'static u32)> {
            expected().iter().map(|(idx, val)| (*idx, val))
        }

        pub(crate) fn as_owned_mut() -> impl Iterator<Item = (char, &'static mut u32)> {
            expected().iter_mut().map(|(idx, val)| (*idx, val))
        }

        pub(crate) fn as_ref_ref() -> impl Iterator<Item = (&'static char, &'static u32)> {
            expected().iter().map(|(idx, val)| (idx, val))
        }

        pub(crate) fn as_ref_mut() -> impl Iterator<Item = (&'static char, &'static mut u32)> {
            expected().iter_mut().map(|(idx, val)| (&*idx, val))
        }

        fn expected() -> &'static mut [(char, u32); 3] {
            unsafe {
                #[allow(static_mut_refs)]
                &mut EXPECTED
            }
        }
    }
}
