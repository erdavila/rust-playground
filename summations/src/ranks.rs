use indexed::{Indexed, IndexedOwned, IndexedRef};

use crate::total_ord::TotalOrdF32;

const FIRST_RANK: u32 = 0;

struct Entry {
    index: usize,
    rank: u32,
}

pub(crate) fn ranks<T>(values: &Vec<T>) -> Vec<u32>
where
    T: Copy + 'static,
    TotalOrdF32: From<T>,
{
    match 3 {
        0 => by_key(values, TotalOrdF32::from_ref),
        1 => by_accessor(values.len(), |i| TotalOrdF32::from_ref(&values[i])),
        2 => {
            let ref_output_idxd = values.as_indexed();
            let owned_output_idxd = ref_output_idxd.into_view(TotalOrdF32::from_ref);
            for_owned_output_indexed(owned_output_idxd)
        }
        3 => {
            let ref_output_idxd = values.as_indexed();
            let owned_output_idxd = ref_output_idxd.into_view(TotalOrdF32::from_ref);
            let idxd_owned = owned_output_idxd.into_indexed_owned();
            for_indexed_owned(idxd_owned)
        }
        _ => panic!("invalid implementation"),
    }
}

fn by_key<T, K: Ord>(values: &[T], mut f: impl FnMut(&T) -> K) -> Vec<u32> {
    let mut entries: Vec<_> = initialize_entries(values.len());

    entries.sort_by_key(|entry| f(&values[entry.index]));

    for i in 1..values.len() {
        let prev_i = i - 1;
        let [prev_entry, entry] = entries.get_disjoint_mut([prev_i, i]).unwrap();
        let mut rank = prev_entry.rank;

        let value = f(&values[entry.index]);
        let prev_value = f(&values[prev_entry.index]);

        if value > prev_value {
            rank += 1;
        }

        entry.rank = rank;
    }

    entries.sort_by_key(|entry| entry.index);

    entries.into_iter().map(|entry| entry.rank).collect()
}

fn by_accessor<T: Ord>(len: usize, mut get: impl FnMut(usize) -> T) -> Vec<u32> {
    let mut entries: Vec<_> = initialize_entries(len);

    entries.sort_by_key(|entry| get(entry.index));

    for i in 1..len {
        let prev_i = i - 1;
        let [prev_entry, entry] = entries.get_disjoint_mut([prev_i, i]).unwrap();
        let mut rank = prev_entry.rank;

        let value = get(entry.index);
        let prev_value = get(prev_entry.index);

        if value > prev_value {
            rank += 1;
        }

        entry.rank = rank;
    }

    entries.sort_by_key(|entry| entry.index);

    entries.into_iter().map(|entry| entry.rank).collect()
}

fn for_owned_output_indexed<T: Ord>(
    values: impl for<'a> Indexed<'a, usize, Output = T>,
) -> Vec<u32> {
    let mut entries: Vec<_> = initialize_entries(values.len());

    entries.sort_by_key(|entry| values.get(entry.index));

    for i in 1..values.len() {
        let prev_i = i - 1;
        let [prev_entry, entry] = entries.get_disjoint_mut([prev_i, i]).unwrap();
        let mut rank = prev_entry.rank;

        let value = values.get(entry.index);
        let prev_value = values.get(prev_entry.index);

        if value > prev_value {
            rank += 1;
        }

        entry.rank = rank;
    }

    entries.sort_by_key(|entry| entry.index);

    entries.into_iter().map(|entry| entry.rank).collect()
}

fn for_indexed_owned<T: Ord>(values: impl IndexedOwned<usize, Output = T>) -> Vec<u32> {
    let mut entries: Vec<_> = initialize_entries(values.len());

    entries.sort_by_key(|entry| values.get(entry.index));

    for i in 1..values.len() {
        let prev_i = i - 1;
        let [prev_entry, entry] = entries.get_disjoint_mut([prev_i, i]).unwrap();
        let mut rank = prev_entry.rank;

        let value = values.get(entry.index);
        let prev_value = values.get(prev_entry.index);

        if value > prev_value {
            rank += 1;
        }

        entry.rank = rank;
    }

    entries.sort_by_key(|entry| entry.index);

    entries.into_iter().map(|entry| entry.rank).collect()
}

fn initialize_entries(len: usize) -> Vec<Entry> {
    (0..len)
        .map(|index| Entry {
            index,
            rank: FIRST_RANK,
        })
        .collect()
}
