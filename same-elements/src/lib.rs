use std::{
    collections::{BTreeMap, HashMap, btree_map, hash_map},
    hash::Hash,
};

macro_rules! update_count {
    ($namespace:ident :: $map_type:ident) => {
        |counts: &mut $map_type<_, _>, key, delta| match counts.entry(key) {
            $namespace::Entry::Occupied(mut occupied) => {
                let count = occupied.get_mut();
                *count += delta;
                if *count == 0 {
                    occupied.remove();
                }
            }
            $namespace::Entry::Vacant(vacant) => {
                vacant.insert(delta);
            }
        }
    };
}

pub fn same_elements_hash<T: Eq + Hash>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> bool {
    same_elements(
        a,
        b,
        HashMap::new(),
        update_count!(hash_map::HashMap),
        HashMap::is_empty,
    )
}

pub fn same_elements_ord<T: Ord>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
) -> bool {
    same_elements(
        a,
        b,
        BTreeMap::new(),
        update_count!(btree_map::BTreeMap),
        BTreeMap::is_empty,
    )
}

fn same_elements<T: Eq, M>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = T>,
    mut counts: M,
    update_count: fn(&mut M, T, i32),
    is_empty: fn(&M) -> bool,
) -> bool {
    let mut a = a.into_iter();
    let mut b = b.into_iter();

    loop {
        match (a.next(), b.next()) {
            (Some(x), Some(y)) => {
                if x != y {
                    update_count(&mut counts, x, 1);
                    update_count(&mut counts, y, -1);
                }
            }
            (None, None) => break,
            _ => return false,
        }
    }

    is_empty(&counts)
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
