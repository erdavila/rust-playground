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
        use std::collections::{BTreeMap, BTreeSet};
        use indexed::Indexed;

        let expected: BTreeMap<_, _> = $expected.into_iter().collect();

        assert_eq!(Indexed::len(&$idxd), expected.len());

        assert_eq!(
            Indexed::indices(&$idxd).into_iter().collect::<BTreeSet<_>>(),
            expected.keys().copied().collect::<BTreeSet<_>>(),
        );

        for (k, v) in expected {
            assert_eq!($idxd.get(k), Some(v));
        }
        assert_eq!($idxd.get($none_index), None);
    }};
}
pub(crate) use assert_indexed;

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
        static EXPECTED: [(char, u32); 3] = super::VALUES;

        pub(crate) fn as_owned_owned() -> impl Iterator<Item = (char, u32)> {
            EXPECTED.into_iter()
        }

        pub(crate) fn as_owned_ref() -> impl Iterator<Item = (char, &'static u32)> {
            EXPECTED.iter().map(|(idx, val)| (*idx, val))
        }

        pub(crate) fn as_ref_ref() -> impl Iterator<Item = (&'static char, &'static u32)> {
            EXPECTED.iter().map(|(idx, val)| (idx, val))
        }
    }
}
