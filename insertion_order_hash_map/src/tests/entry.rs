use std::cell::RefCell;

use crate::{Entry, InsertionOrderHashMap, OccupiedEntry, VacantEntry};

use super::consistency;

#[test]
fn test_entry_occupied() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let entry = iohm.entry("A");

    match entry {
        Entry::Occupied(_) => (),
        Entry::Vacant(_) => panic!("entry should be occupied"),
    }
    consistency::assert(&iohm);
}

#[test]
fn test_entry_vacant() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();

    let entry = iohm.entry("B");

    match entry {
        Entry::Occupied(_) => panic!("entry should be vacant"),
        Entry::Vacant(_) => (),
    }
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_insert_on_occupied() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let entry = iohm.entry("A");

    let value = entry.or_insert(2);

    assert_eq!(value, &1);
    assert_eq!(iohm.get(&"A"), Some(&1));
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_insert_on_vacant() {
    let mut iohm = InsertionOrderHashMap::new();
    let entry = iohm.entry("A");

    let value = entry.or_insert(2);

    assert_eq!(value, &2);
    assert_eq!(iohm.get(&"A"), Some(&2));
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_insert_with_on_occupied() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let entry = iohm.entry("A");

    let value = entry.or_insert_with(|| 2);

    assert_eq!(value, &1);
    assert_eq!(iohm.get(&"A"), Some(&1));
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_insert_with_on_vacant() {
    let mut iohm = InsertionOrderHashMap::new();
    let entry = iohm.entry("A");

    let value = entry.or_insert_with(|| 2);

    assert_eq!(value, &2);
    assert_eq!(iohm.get(&"A"), Some(&2));
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_insert_with_key_on_occupied() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let entry = iohm.entry("A");
    let closure_was_called = RefCell::new(false);

    let value = entry.or_insert_with_key(|key| {
        assert_eq!(*key, "A");
        *closure_was_called.borrow_mut() = true;
        2
    });

    assert_eq!(value, &1);
    assert_eq!(iohm.get(&"A"), Some(&1));
    assert!(!*closure_was_called.borrow());
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_insert_with_key_on_vacant() {
    let mut iohm = InsertionOrderHashMap::new();
    let entry = iohm.entry("A");
    let closure_was_called = RefCell::new(false);

    let value = entry.or_insert_with_key(|key| {
        assert_eq!(*key, "A");
        *closure_was_called.borrow_mut() = true;
        2
    });

    assert_eq!(value, &2);
    assert_eq!(iohm.get(&"A"), Some(&2));
    assert!(*closure_was_called.borrow());
    consistency::assert(&iohm);
}

#[test]
fn test_entry_key_on_occupied() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let entry = iohm.entry("A");

    let key = entry.key();

    assert_eq!(*key, "A");
    consistency::assert(&iohm);
}

#[test]
fn test_entry_key_on_vacant() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    let entry = iohm.entry("A");

    let key = entry.key();

    assert_eq!(*key, "A");
    consistency::assert(&iohm);
}

#[test]
fn test_entry_and_modify_on_occupied() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let entry = iohm.entry("A");

    let entry = entry.and_modify(|value| *value += 1);

    match entry {
        Entry::Occupied(occupied_entry) => assert_eq!(occupied_entry.get(), &2),
        Entry::Vacant(_) => panic!("Entry should be Occupied(_)"),
    }
    assert_eq!(iohm.get(&"A"), Some(&2));
    consistency::assert(&iohm);
}

#[test]
fn test_entry_and_modify_on_vacant() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    let entry = iohm.entry("A");

    let entry = entry.and_modify(|value| *value += 1);

    match entry {
        Entry::Occupied(_) => panic!("Entry should be Vacant(_)"),
        Entry::Vacant(_) => (),
    }
    assert_eq!(iohm.get(&"A"), None);
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_default_on_occupied() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    iohm.insert("A", 1);
    let entry = iohm.entry("A");

    let value = entry.or_default();

    assert_eq!(value, &1);
    consistency::assert(&iohm);
}

#[test]
fn test_entry_or_default_on_vacant() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    let entry = iohm.entry("A");

    let value = entry.or_default();

    assert_eq!(value, &0);
    assert_eq!(iohm.get(&"A"), Some(&0));
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_key() {
    let mut iohm = InsertionOrderHashMap::new();
    let occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let key = occupied_entry.key();

    assert_eq!(key, &"A");
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_remove_entry() {
    let mut iohm = InsertionOrderHashMap::new();
    let occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let (key, value) = occupied_entry.remove_entry();

    assert_eq!(key, "A");
    assert_eq!(value, 1);
    assert!(iohm.nodes.is_empty());
    assert!(iohm.order.is_none());
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_get() {
    let mut iohm = InsertionOrderHashMap::new();
    let occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let value: &i32 = occupied_entry.get();

    assert_eq!(*value, 1);
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_get_mut() {
    let mut iohm = InsertionOrderHashMap::new();
    let mut occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let value: &mut i32 = occupied_entry.get_mut();

    assert_eq!(*value, 1);
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_into_mut() {
    let mut iohm = InsertionOrderHashMap::new();
    let occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let value: &mut i32 = occupied_entry.into_mut();

    assert_eq!(*value, 1);
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_insert() {
    let mut iohm = InsertionOrderHashMap::new();
    let mut occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let value = occupied_entry.insert(2);

    assert_eq!(value, 1);
    assert_eq!(iohm.get(&"A"), Some(&2));
    consistency::assert(&iohm);
}

#[test]
fn test_occupied_entry_remove() {
    let mut iohm = InsertionOrderHashMap::new();
    let occupied_entry = insert_and_get_occupied_entry(&mut iohm, "A", 1);

    let value = occupied_entry.remove();

    assert_eq!(value, 1);
    assert_eq!(iohm.get(&"A"), None);
    consistency::assert(&iohm);
}

#[test]
fn test_vacant_entry_key() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    let vacant_entry = get_vacant_entry(&mut iohm, "A");

    let key = vacant_entry.key();

    assert_eq!(key, &"A");
    consistency::assert(&iohm);
}

#[test]
fn test_vacant_entry_into_key() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    let vacant_entry = get_vacant_entry(&mut iohm, "A");

    let key = vacant_entry.into_key();

    assert_eq!(key, "A");
    consistency::assert(&iohm);
}

#[test]
fn test_vacant_entry_insert() {
    let mut iohm = InsertionOrderHashMap::<&str, i32>::new();
    let vacant_entry = get_vacant_entry(&mut iohm, "A");

    let value = vacant_entry.insert(1);

    assert_eq!(value, &1);
    assert_eq!(iohm.get(&"A"), Some(&1));
    consistency::assert(&iohm);
}

fn insert_and_get_occupied_entry<'a, K, V>(
    iohm: &'a mut InsertionOrderHashMap<K, V>,
    key: K,
    value: V,
) -> OccupiedEntry<'a, K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    iohm.insert(key.clone(), value);
    match iohm.entry(key) {
        Entry::Occupied(occupied_entry) => occupied_entry,
        Entry::Vacant(_) => panic!("Entry should be Occupied(_)"),
    }
}

fn get_vacant_entry<'a, K, V>(
    iohm: &'a mut InsertionOrderHashMap<K, V>,
    key: K,
) -> VacantEntry<K, V>
where
    K: std::hash::Hash + Eq,
{
    match iohm.entry(key) {
        Entry::Occupied(_) => panic!("Entry should be Vacant(_)"),
        Entry::Vacant(occupied_entry) => occupied_entry,
    }
}
