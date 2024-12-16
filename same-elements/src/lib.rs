use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    hash::Hash,
};

pub fn same_elements_hash<T: Eq + Hash>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> bool {
    let mut map = HashMap::new();

    for x in a {
        map.entry(x)
            .and_modify(|count| *count += 1)
            .or_insert(0usize);
    }

    for y in b {
        if let hash_map::Entry::Occupied(mut e) = map.entry(y) {
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
}

pub fn same_elements_ord<T: Ord>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> bool {
    let mut map = BTreeMap::new();

    for x in a {
        map.entry(x)
            .and_modify(|count| *count += 1)
            .or_insert(0usize);
    }

    for y in b {
        if let btree_map::Entry::Occupied(mut e) = map.entry(y) {
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
