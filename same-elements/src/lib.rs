use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    hash::Hash,
};

macro_rules! same_elements {
    ($a:ident, $b:ident, $map:expr, $namespace:ident) => {{
        let a = $a;
        let b = $b;
        let mut map = $map;

        for x in a {
            map.entry(x)
                .and_modify(|count| *count += 1)
                .or_insert(0usize);
        }

        for y in b {
            if let $namespace::Entry::Occupied(mut e) = map.entry(y) {
                let count = e.get_mut();
                if *count == 0 {
                    e.remove();
                } else {
                    *count -= 1;
                }
            } else {
                return false;
            }
        }

        map.is_empty()
    }};
}

pub fn same_elements_hash<T: Eq + Hash>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> bool {
    same_elements!(a, b, HashMap::new(), hash_map)
}

pub fn same_elements_ord<T: Ord>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> bool {
    same_elements!(a, b, BTreeMap::new(), btree_map)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[template]
    #[rstest]
    #[case([], [], true)]
    #[case([], [1], false)]
    #[case([1], [], false)]
    #[case([1], [1, 1], false)]
    #[case([1, 1], [1], false)]
    #[case([1, 2], [1, 2], true)]
    #[case([1, 2], [2, 1], true)]
    #[case([1, 2], [1, 2, 2], false)]
    #[case([1, 1, 2], [1, 2, 1], true)]
    #[case([1, 1, 2], [1, 2, 2], false)]
    fn cases(
        #[case] a: impl IntoIterator<Item = u32>,
        #[case] b: impl IntoIterator<Item = u32>,
        #[case] expected: bool,
    ) {
    }

    #[apply(cases)]
    fn same_elements_hash(
        #[case] a: impl IntoIterator<Item = u32>,
        #[case] b: impl IntoIterator<Item = u32>,
        #[case] expected: bool,
    ) {
        let output = crate::same_elements_hash(a, b);

        assert_eq!(output, expected);
    }

    #[apply(cases)]
    fn same_elements_ord(
        #[case] a: impl IntoIterator<Item = u32>,
        #[case] b: impl IntoIterator<Item = u32>,
        #[case] expected: bool,
    ) {
        let output = crate::same_elements_ord(a, b);

        assert_eq!(output, expected);
    }
}
