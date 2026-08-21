use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::mods::asserts::entries::{NONE_INDEX, VALUES, expected};

mod mods;

#[test]
fn enumerated() {
    let values = VALUES.map(|(_, val)| val);

    macro_rules! assert_indexed {
        ($idxd:expr) => {
            $crate::mods::asserts::assert_indexed!($idxd, values.iter().enumerate(), values.len());
        };
    }

    let slice = values.as_slice();
    assert_indexed!(slice);

    let array = values;
    assert_indexed!(array);

    let vec = Vec::from(values);
    assert_indexed!(vec);

    let vec_deque = VecDeque::from(values);
    assert_indexed!(vec_deque);
}

#[test]
fn map() {
    let values = VALUES;

    macro_rules! assert_indexed {
        ($idxd:expr) => {
            $crate::mods::asserts::assert_indexed!($idxd, expected::as_ref_ref(), &NONE_INDEX);
        };
    }

    let btree_map = BTreeMap::from(values);
    assert_indexed!(btree_map);

    let hash_map = HashMap::from(values);
    assert_indexed!(hash_map);
}
