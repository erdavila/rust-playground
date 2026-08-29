use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::mods::asserts::entries::{NONE_INDEX, VALUES, expected};

mod mods;

#[test]
fn enumerated() {
    let mut values = VALUES.map(|(_, val)| val);
    let mut values2 = values;
    let none_index = values.len();

    macro_rules! assert_indexed_mut {
        ($idxd_mut:ident) => {{
            $crate::mods::asserts::assert_indexed_mut!(
                $idxd_mut,
                values2.iter_mut().enumerate(),
                none_index,
            );
        }};
    }

    let mut slice = values.as_mut_slice();
    assert_indexed_mut!(slice);

    let mut array = values;
    assert_indexed_mut!(array);

    let mut vec = Vec::from(values);
    assert_indexed_mut!(vec);

    let mut vec_deque = VecDeque::from(values);
    assert_indexed_mut!(vec_deque);
}

#[test]
fn map() {
    let values = VALUES;

    macro_rules! assert_indexed_mut {
        ($idxd_mut:ident) => {{
            $crate::mods::asserts::assert_indexed_mut!(
                $idxd_mut,
                expected::as_ref_mut(),
                &NONE_INDEX,
            );
        }};
    }

    let mut btree_map = BTreeMap::from(values);
    assert_indexed_mut!(btree_map);

    let mut hash_map = HashMap::from(values);
    assert_indexed_mut!(hash_map);
}
