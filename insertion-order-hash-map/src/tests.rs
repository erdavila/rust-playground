use alloc::string::{String, ToString as _};
use alloc::vec::Vec;
use alloc::{format, vec};

use super::*;

mod consistency;
mod entry;
mod stress_test;

#[test]
fn test_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    assert_eq!(iohm.len(), 0);
    assert!(iohm.is_empty());
    assert!(iohm.nodes.is_empty());
    assert!(iohm.order.is_none());
    consistency::assert(&iohm);
}

#[test]
fn test_get_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A".to_string(), 1);
    let iohm = as_immutable(iohm);

    let result1 = iohm.get(&"A".to_string());
    let result2 = iohm.get("A");

    assert_eq!(result1, Some(&1));
    assert_eq!(result2, result1);
    consistency::assert(&iohm);
}

#[test]
fn test_get_non_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A".to_string(), 1);
    let iohm = as_immutable(iohm);

    let result1 = iohm.get(&"B".to_string());
    let result2 = iohm.get("B");

    assert!(result1.is_none());
    assert_eq!(result2, result1);
    consistency::assert(&iohm);
}

#[test]
fn test_first_key_value_on_empty() {
    let iohm: InsertionOrderHashMap<&str, i32> = InsertionOrderHashMap::new();

    let result = iohm.first_key_value();

    assert!(result.is_none());
}

#[test]
fn test_first_key_value_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let result = iohm.first_key_value();

    assert_eq!(result, Some((&"A", &1)));
}

#[test]
fn test_first_entry_on_empty() {
    let mut iohm: InsertionOrderHashMap<&str, i32> = InsertionOrderHashMap::new();

    let result = iohm.first_entry();

    assert!(result.is_none());
}

#[test]
fn test_first_entry_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.first_entry();

    assert!(result.is_some());
    let occupied_entry = result.unwrap();
    //println!("{}", iohm.len()); // should not compile!
    assert_eq!(occupied_entry.key(), &"A");
    assert_eq!(occupied_entry.get(), &1);
}

#[test]
fn test_pop_first_on_empty() {
    let mut iohm: InsertionOrderHashMap<&str, i32> = InsertionOrderHashMap::new();

    let result = iohm.pop_first();

    consistency::assert(&iohm);
    assert!(result.is_none());
}

#[test]
fn test_pop_first_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.pop_first();

    consistency::assert(&iohm);
    assert_eq!(iohm.len(), 2);
    assert!(result.is_some());
    let (k, v) = result.unwrap();
    assert_eq!(k, "A");
    assert_eq!(v, 1);
}

#[test]
fn test_last_key_value_on_empty() {
    let iohm: InsertionOrderHashMap<&str, i32> = InsertionOrderHashMap::new();

    let result = iohm.last_key_value();

    assert!(result.is_none());
}

#[test]
fn test_last_key_value_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let result = iohm.last_key_value();

    assert_eq!(result, Some((&"C", &3)));
}

#[test]
fn test_last_entry_on_empty() {
    let mut iohm: InsertionOrderHashMap<&str, i32> = InsertionOrderHashMap::new();

    let result = iohm.last_entry();

    assert!(result.is_none());
}

#[test]
fn test_last_entry_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.last_entry();

    assert!(result.is_some());
    let occupied_entry = result.unwrap();
    //println!("{}", iohm.len()); // should not compile!
    assert_eq!(occupied_entry.key(), &"C");
    assert_eq!(occupied_entry.get(), &3);
}

#[test]
fn test_pop_last_on_empty() {
    let mut iohm: InsertionOrderHashMap<&str, i32> = InsertionOrderHashMap::new();

    let result = iohm.pop_last();

    consistency::assert(&iohm);
    assert!(result.is_none());
}

#[test]
fn test_pop_last_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.pop_last();

    consistency::assert(&iohm);
    assert_eq!(iohm.len(), 2);
    assert!(result.is_some());
    let (k, v) = result.unwrap();
    assert_eq!(k, "C");
    assert_eq!(v, 3);
}

#[test]
fn test_insert_on_empty() {
    let mut iohm = InsertionOrderHashMap::new();

    let result = iohm.insert("A", 1);

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 1);
    let node_ptr = iohm.nodes[&KeyPtr::new(&"A")];
    assert_eq!(node_ptr.key(), &"A");
    assert_eq!(node_ptr.value(), &1);
    assert_first_node(&iohm, node_ptr);
    assert_last_node(&iohm, node_ptr);
    consistency::assert(&iohm);
}

#[test]
fn test_insert_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.insert("B", 2);

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 2);
    let node_ptr_a = iohm.nodes[&KeyPtr::new(&"A")];
    let node_ptr_b = iohm.nodes[&KeyPtr::new(&"B")];
    assert_eq!(node_ptr_b.key(), &"B");
    assert_eq!(node_ptr_b.value(), &2);
    assert_linked_nodes(node_ptr_a, node_ptr_b);
    assert_last_node(&iohm, node_ptr_b);
    consistency::assert(&iohm);
}

#[test]
fn test_insert_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.insert("A", 2);

    let iohm = as_immutable(iohm);
    assert_eq!(result, Some(1));
    assert_eq!(iohm.nodes.len(), 1);
    let node_ptr = iohm.nodes[&KeyPtr::new(&"A")];
    assert_eq!(node_ptr.key(), &"A");
    assert_eq!(node_ptr.value(), &2);
    assert_first_node(&iohm, node_ptr);
    assert_last_node(&iohm, node_ptr);
    consistency::assert(&iohm);
}

#[test]
fn test_remove_first() {
    fn do_test<F>(f: F)
    where
        F: FnOnce(&mut InsertionOrderHashMap<String, i32>) -> Option<i32>,
    {
        let mut iohm = InsertionOrderHashMap::new();
        iohm.insert("A".to_string(), 1);
        iohm.insert("B".to_string(), 2);
        iohm.insert("C".to_string(), 3);

        let result = f(&mut iohm);

        let iohm = as_immutable(iohm);
        assert_eq!(result, Some(1));
        assert_eq!(iohm.nodes.len(), 2);
        let node_ptr = iohm.nodes[&KeyPtr::new(&"B".to_string())];
        assert_first_node(&iohm, node_ptr);
        consistency::assert(&iohm);
    }

    do_test(|iohm| iohm.remove(&"A".to_string()));
    do_test(|iohm| iohm.remove("A"));
}

#[test]
fn test_remove_last() {
    fn do_test<F>(f: F)
    where
        F: FnOnce(&mut InsertionOrderHashMap<String, i32>) -> Option<i32>,
    {
        let mut iohm = InsertionOrderHashMap::new();
        iohm.insert("A".to_string(), 1);
        iohm.insert("B".to_string(), 2);
        iohm.insert("C".to_string(), 3);

        let result = f(&mut iohm);

        let iohm = as_immutable(iohm);
        assert_eq!(result, Some(3));
        assert_eq!(iohm.nodes.len(), 2);
        let node_ptr = iohm.nodes[&KeyPtr::new(&"B".to_string())];
        assert_last_node(&iohm, node_ptr);
        consistency::assert(&iohm);
    }

    do_test(|iohm| iohm.remove(&"C".to_string()));
    do_test(|iohm| iohm.remove("C"));
}

#[test]
fn test_remove_in_the_middle() {
    fn do_test<F>(f: F)
    where
        F: FnOnce(&mut InsertionOrderHashMap<String, i32>) -> Option<i32>,
    {
        let mut iohm = InsertionOrderHashMap::new();
        iohm.insert("A".to_string(), 1);
        iohm.insert("B".to_string(), 2);
        iohm.insert("C".to_string(), 3);

        let result = f(&mut iohm);

        let iohm = as_immutable(iohm);
        assert_eq!(result, Some(2));
        assert_eq!(iohm.nodes.len(), 2);
        let node_ptr_a = iohm.nodes[&KeyPtr::new(&"A".to_string())];
        let node_ptr_c = iohm.nodes[&KeyPtr::new(&"C".to_string())];
        assert_linked_nodes(node_ptr_a, node_ptr_c);
        consistency::assert(&iohm);
    }

    do_test(|iohm| iohm.remove(&"B".to_string()));
    do_test(|iohm| iohm.remove("B"));
}

#[test]
fn test_remove_single_item() {
    fn do_test<F>(f: F)
    where
        F: FnOnce(&mut InsertionOrderHashMap<String, i32>) -> Option<i32>,
    {
        let mut iohm = InsertionOrderHashMap::new();
        iohm.insert("A".to_string(), 1);

        let result = f(&mut iohm);

        let iohm = as_immutable(iohm);
        assert_eq!(result, Some(1));
        assert!(iohm.nodes.is_empty());
        assert!(iohm.order.is_none());
        consistency::assert(&iohm);
    }

    do_test(|iohm| iohm.remove(&"A".to_string()));
    do_test(|iohm| iohm.remove("A"));
}

#[test]
fn test_remove_non_existing_key() {
    fn do_test<F>(f: F)
    where
        F: FnOnce(&mut InsertionOrderHashMap<String, i32>) -> Option<i32>,
    {
        let mut iohm = InsertionOrderHashMap::new();
        iohm.insert("A".to_string(), 1);

        let result = f(&mut iohm);

        let iohm = as_immutable(iohm);
        assert!(result.is_none());
        assert_eq!(iohm.nodes.len(), 1);
        consistency::assert(&iohm);
    }

    do_test(|iohm| iohm.remove(&"B".to_string()));
    do_test(|iohm| iohm.remove("B"));
}

#[test]
fn test_clear() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    iohm.clear();

    let iohm = as_immutable(iohm);
    assert!(iohm.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_keys_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let keys = iohm.keys();

    let keys_vec: Vec<_> = keys.collect();
    assert!(keys_vec.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_keys_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let keys = iohm.keys();

    let keys_vec: Vec<_> = keys.collect();
    assert_eq!(keys_vec, vec![&"A", &"B", &"C"]);
    consistency::assert(&iohm);
}

#[test]
fn test_keys_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut keys = iohm.keys();

    assert_eq!(keys.len(), 3);
    assert_eq!(keys.next(), Some(&"A"));
    assert_eq!(keys.len(), 2);
    assert_eq!(keys.next(), Some(&"B"));
    assert_eq!(keys.len(), 1);
    assert_eq!(keys.next(), Some(&"C"));
    assert_eq!(keys.len(), 0);
    assert_eq!(keys.next(), None);
}

#[test]
fn test_keys_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut keys = iohm.keys();

    assert_eq!(keys.len(), 3);
    assert_eq!(keys.next_back(), Some(&"C"));
    assert_eq!(keys.len(), 2);
    assert_eq!(keys.next_back(), Some(&"B"));
    assert_eq!(keys.len(), 1);
    assert_eq!(keys.next_back(), Some(&"A"));
    assert_eq!(keys.len(), 0);
    assert_eq!(keys.next_back(), None);
}

#[test]
fn test_into_keys_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let keys = iohm.into_keys();

    let keys_vec: Vec<_> = keys.collect();
    assert!(keys_vec.is_empty());
}

#[test]
fn test_into_keys_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let keys = iohm.into_keys();

    let keys_vec: Vec<_> = keys.collect();
    assert_eq!(keys_vec, vec!["A", "B", "C"]);
}

#[test]
fn test_into_keys_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut keys = iohm.into_keys();

    assert_eq!(keys.len(), 3);
    assert_eq!(keys.next(), Some("A"));
    assert_eq!(keys.len(), 2);
    assert_eq!(keys.next(), Some("B"));
    assert_eq!(keys.len(), 1);
    assert_eq!(keys.next(), Some("C"));
    assert_eq!(keys.len(), 0);
    assert_eq!(keys.next(), None);
}

#[test]
fn test_into_keys_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut keys = iohm.into_keys();

    assert_eq!(keys.len(), 3);
    assert_eq!(keys.next_back(), Some("C"));
    assert_eq!(keys.len(), 2);
    assert_eq!(keys.next_back(), Some("B"));
    assert_eq!(keys.len(), 1);
    assert_eq!(keys.next_back(), Some("A"));
    assert_eq!(keys.len(), 0);
    assert_eq!(keys.next_back(), None);
}

#[test]
fn test_values_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let values = iohm.values();

    let values_vec: Vec<_> = values.collect();
    assert!(values_vec.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_values_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let values = iohm.values();

    let values_vec: Vec<_> = values.collect();
    assert_eq!(values_vec, vec![&1, &2, &3]);
    consistency::assert(&iohm);
}

#[test]
fn test_values_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut values = iohm.values();

    assert_eq!(values.len(), 3);
    assert_eq!(values.next(), Some(&1));
    assert_eq!(values.len(), 2);
    assert_eq!(values.next(), Some(&2));
    assert_eq!(values.len(), 1);
    assert_eq!(values.next(), Some(&3));
    assert_eq!(values.len(), 0);
    assert_eq!(values.next(), None);
}

#[test]
fn test_values_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut values = iohm.values();

    assert_eq!(values.len(), 3);
    assert_eq!(values.next_back(), Some(&3));
    assert_eq!(values.len(), 2);
    assert_eq!(values.next_back(), Some(&2));
    assert_eq!(values.len(), 1);
    assert_eq!(values.next_back(), Some(&1));
    assert_eq!(values.len(), 0);
    assert_eq!(values.next(), None);
}

#[test]
fn test_values_mut_on_empty() {
    let mut iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let values = iohm.values_mut();

    let values_vec: Vec<_> = values.collect();
    assert!(values_vec.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_values_mut_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    for n in iohm.values_mut() {
        *n += 10;
    }

    let values_vec: Vec<_> = iohm.values().collect();
    assert_eq!(values_vec, vec![&11, &12, &13]);
    consistency::assert(&iohm);
}

#[test]
fn test_values_mut_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut values = iohm.values_mut();

    assert_eq!(values.len(), 3);
    assert_eq!(values.next(), Some(&mut 1));
    assert_eq!(values.len(), 2);
    assert_eq!(values.next(), Some(&mut 2));
    assert_eq!(values.len(), 1);
    assert_eq!(values.next(), Some(&mut 3));
    assert_eq!(values.len(), 0);
    assert_eq!(values.next(), None);
}

#[test]
fn test_values_mut_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut values = iohm.values_mut();

    assert_eq!(values.len(), 3);
    assert_eq!(values.next_back(), Some(&mut 3));
    assert_eq!(values.len(), 2);
    assert_eq!(values.next_back(), Some(&mut 2));
    assert_eq!(values.len(), 1);
    assert_eq!(values.next_back(), Some(&mut 1));
    assert_eq!(values.len(), 0);
    assert_eq!(values.next_back(), None);
}

#[test]
fn test_into_values_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let values = iohm.into_values();

    let values_vec: Vec<_> = values.collect();
    assert!(values_vec.is_empty());
}

#[test]
fn test_into_values_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let values = iohm.into_values();

    let values_vec: Vec<_> = values.collect();
    assert_eq!(values_vec, vec![1, 2, 3]);
}

#[test]
fn test_into_values_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut values = iohm.into_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values.next(), Some(1));
    assert_eq!(values.len(), 2);
    assert_eq!(values.next(), Some(2));
    assert_eq!(values.len(), 1);
    assert_eq!(values.next(), Some(3));
    assert_eq!(values.len(), 0);
    assert_eq!(values.next(), None);
}

#[test]
fn test_into_values_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut values = iohm.into_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values.next_back(), Some(3));
    assert_eq!(values.len(), 2);
    assert_eq!(values.next_back(), Some(2));
    assert_eq!(values.len(), 1);
    assert_eq!(values.next_back(), Some(1));
    assert_eq!(values.len(), 0);
    assert_eq!(values.next_back(), None);
}

#[test]
fn test_iter_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let iter = iohm.iter();

    let iter_vec: Vec<_> = iter.collect();
    assert!(iter_vec.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_iter_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let iter = iohm.iter();

    let iter_vec: Vec<_> = iter.collect();
    assert_eq!(iter_vec, vec![(&"A", &1), (&"B", &2), (&"C", &3)]);
    consistency::assert(&iohm);
}

#[test]
fn test_iter_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut iter = iohm.iter();

    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some((&"A", &1)));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some((&"B", &2)));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some((&"C", &3)));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let mut iter = iohm.iter();

    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next_back(), Some((&"C", &3)));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next_back(), Some((&"B", &2)));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next_back(), Some((&"A", &1)));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_on_empty() {
    let mut iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let iter = iohm.iter_mut();

    let iter_vec: Vec<_> = iter.collect();
    assert!(iter_vec.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_iter_mut_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1i32);
    iohm.insert("B", 2i32);
    iohm.insert("C", 3i32);

    for (k, v) in &mut iohm {
        *v *= 100;
        *v += k.chars().next().unwrap() as i32;
    }

    let iter_vec: Vec<_> = iohm.values().collect();
    assert_eq!(iter_vec, vec![&165, &266, &367]);
    consistency::assert(&iohm);
}

#[test]
fn test_iter_mut_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut iter = iohm.iter_mut();

    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some((&"A", &mut 1)));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some((&"B", &mut 2)));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some((&"C", &mut 3)));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut iter = iohm.iter_mut();

    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next_back(), Some((&"C", &mut 3)));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next_back(), Some((&"B", &mut 2)));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next_back(), Some((&"A", &mut 1)));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn test_drain_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut drain = iohm.drain();

    // println!("{}", iohm.len()); // should not compile!
    assert_eq!(drain.len(), 3);
    assert_eq!(drain.next(), Some(("A", 1)));
    assert_eq!(drain.len(), 2);
    assert_eq!(drain.next(), Some(("B", 2)));
    assert_eq!(drain.len(), 1);
    assert_eq!(drain.next(), Some(("C", 3)));
    assert_eq!(drain.len(), 0);
    assert_eq!(drain.next(), None);
    mem::drop(drain);
    assert!(iohm.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_drain_reverse_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut drain = iohm.drain();

    // println!("{}", iohm.len()); // should not compile!
    assert_eq!(drain.len(), 3);
    assert_eq!(drain.next_back(), Some(("C", 3)));
    assert_eq!(drain.len(), 2);
    assert_eq!(drain.next_back(), Some(("B", 2)));
    assert_eq!(drain.len(), 1);
    assert_eq!(drain.next_back(), Some(("A", 1)));
    assert_eq!(drain.len(), 0);
    assert_eq!(drain.next_back(), None);
    mem::drop(drain);
    assert!(iohm.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_drain_drop() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut drain = iohm.drain();

    assert_eq!(drain.len(), 3);
    assert_eq!(drain.next(), Some(("A", 1)));
    assert_eq!(drain.len(), 2);
    mem::drop(drain);
    assert!(iohm.is_empty());
    consistency::assert(&iohm);
}

#[test]
fn test_clone() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let result = iohm.clone();

    consistency::assert(&result);
    assert_eq!(result.len(), 3);
    let vec: Vec<_> = result.iter().collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &2), (&"C", &3)]);
}

#[test]
fn test_debug() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A".to_string(), 1);
    iohm.insert("B".to_string(), 2);
    iohm.insert("C".to_string(), 3);
    let iohm = as_immutable(iohm);

    let result = format!("{iohm:#?}");

    assert_eq!(
        result,
        "InsertionOrderHashMap {
    \"A\": 1,
    \"B\": 2,
    \"C\": 3,
}"
    );
}

#[test]
fn test_extend_copying() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", -2);
    let extension = [(&"C", &-3), (&"B", &2), (&"C", &3)].into_iter();

    iohm.extend(extension);

    consistency::assert(&iohm);
    assert_eq!(iohm.len(), 3);
    let vec: Vec<_> = iohm.iter().collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &2), (&"C", &3)]);
}

#[test]
fn test_extend_moving() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", -2);
    let extension = [("C", -3), ("B", 2), ("C", 3)].into_iter();

    iohm.extend(extension);

    consistency::assert(&iohm);
    assert_eq!(iohm.len(), 3);
    let vec: Vec<_> = iohm.iter().collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &2), (&"C", &3)]);
}

#[test]
fn test_from() {
    let iohm = InsertionOrderHashMap::from([("A", 1), ("B", -2), ("C", 3), ("B", 2)]);

    consistency::assert(&iohm);
    assert_eq!(iohm.len(), 3);
    let vec: Vec<_> = iohm.iter().collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &2), (&"C", &3),]);
}

#[test]
fn test_from_iter() {
    let iohm = InsertionOrderHashMap::from_iter([("A", 1), ("B", -2), ("C", 3), ("B", 2)]);

    consistency::assert(&iohm);
    assert_eq!(iohm.len(), 3);
    let vec: Vec<_> = iohm.iter().collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &2), (&"C", &3),]);
}

#[test]
fn test_index_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A".to_string(), 1);
    let iohm = as_immutable(iohm);

    let result1 = iohm["A"];
    let result2 = iohm[&"A".to_string()];

    assert_eq!(result1, 1);
    assert_eq!(result2, 1);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_non_existing_key_1() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A".to_string(), 1);
    let iohm = as_immutable(iohm);

    let _ = iohm["B"];
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_non_existing_key_2() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A".to_string(), 1);
    let iohm = as_immutable(iohm);

    let _ = iohm[&"B".to_string()];
}

#[test]
fn test_into_iterator_ref() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let result = (&iohm).into_iter();

    let vec: Vec<_> = result.collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &2), (&"C", &3)]);
    assert_eq!(iohm.len(), 3);
}

#[test]
fn test_into_iterator_mut() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = (&mut iohm).into_iter();

    let vec: Vec<_> = result.collect();
    assert_eq!(vec, vec![(&"A", &mut 1), (&"B", &mut 2), (&"C", &mut 3)]);
    assert_eq!(iohm.len(), 3);
}

#[test]
fn test_into_iterator_consuming() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    let iohm = as_immutable(iohm);

    let result = iohm.into_iter();

    let vec: Vec<_> = result.collect();
    assert_eq!(vec, vec![("A", 1), ("B", 2), ("C", 3)]);
}

#[test]
fn test_retain() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);
    iohm.insert("D", 4);
    iohm.insert("E", 5);

    iohm.retain(|key, value| {
        if *key == "B" {
            *value += 10;
        }

        *key != "C" && *key != "D"
    });

    consistency::assert(&iohm);
    let vec: Vec<_> = iohm.iter().collect();
    assert_eq!(vec, vec![(&"A", &1), (&"B", &12), (&"E", &5),]);
}

fn assert_first_node<K, V>(iohm: &InsertionOrderHashMap<K, V>, node_ptr: NodePtr<K, V>) {
    let order = iohm.order.as_ref().expect("order should not be None");
    assert_eq!(order.first, node_ptr);

    assert!(node_ptr.prev().is_none());
}

fn assert_last_node<K, V>(iohm: &InsertionOrderHashMap<K, V>, node_ptr: NodePtr<K, V>) {
    let order = iohm.order.as_ref().expect("order should not be None");
    assert_eq!(order.last, node_ptr);

    assert!(node_ptr.next().is_none());
}

fn assert_linked_nodes<K, V>(node_before: NodePtr<K, V>, node_after: NodePtr<K, V>) {
    let node_before_next = node_before
        .next()
        .expect("node_before.next should not be None");
    assert_eq!(node_before_next, node_after);

    let node_after_prev = node_after
        .prev()
        .expect("node_after.prev should not be None");
    assert_eq!(node_after_prev, node_before);
}

fn as_immutable<T>(value: T) -> T {
    value
}
