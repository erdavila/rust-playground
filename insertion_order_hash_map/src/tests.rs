use std::ptr;

use super::*;

mod consistency;
mod entry;
mod stress_test;

#[test]
fn test_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

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
fn test_insert_on_empty() {
    let mut iohm = InsertionOrderHashMap::new();

    let result = iohm.insert("A", 1);

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 1);
    let node: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"A")];
    assert_eq!(node.key, "A");
    assert_eq!(node.value, 1);
    assert_first_node(&iohm, node);
    assert_last_node(&iohm, node);
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
    let node_a: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"A")];
    let node_b: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"B")];
    assert_eq!(node_b.key, "B");
    assert_eq!(node_b.value, 2);
    assert_linked_nodes(node_a, node_b);
    assert_last_node(&iohm, node_b);
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
    let node: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"A")];
    assert_eq!(node.key, "A");
    assert_eq!(node.value, 2);
    assert_first_node(&iohm, node);
    assert_last_node(&iohm, node);
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
        let node: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"B".to_string())];
        assert_first_node(&iohm, node);
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
        let node: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"B".to_string())];
        assert_last_node(&iohm, node);
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
        let node_a: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"A".to_string())];
        let node_c: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"C".to_string())];
        assert_linked_nodes(node_a, node_c);
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

    consistency::assert_nodes_and_order_from_first_node_option(
        &keys.it.iohm().nodes,
        &keys.it.next_node,
    );
    assert_eq!(keys.len(), 3);
    assert_eq!(keys.next(), Some("A"));
    consistency::assert_nodes_and_order_from_first_node_option(
        &keys.it.iohm().nodes,
        &keys.it.next_node,
    );
    assert_eq!(keys.len(), 2);
    assert_eq!(keys.next(), Some("B"));
    consistency::assert_nodes_and_order_from_first_node_option(
        &keys.it.iohm().nodes,
        &keys.it.next_node,
    );
    assert_eq!(keys.len(), 1);
    assert_eq!(keys.next(), Some("C"));
    consistency::assert_nodes_and_order_from_first_node_option(
        &keys.it.iohm().nodes,
        &keys.it.next_node,
    );
    assert_eq!(keys.len(), 0);
    assert_eq!(keys.next(), None);
    consistency::assert_nodes_and_order_from_first_node_option(
        &keys.it.iohm().nodes,
        &keys.it.next_node,
    );
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

    consistency::assert_nodes_and_order_from_first_node_option(
        &values.it.iohm().nodes,
        &values.it.next_node,
    );
    assert_eq!(values.len(), 3);
    assert_eq!(values.next(), Some(1));
    consistency::assert_nodes_and_order_from_first_node_option(
        &values.it.iohm().nodes,
        &values.it.next_node,
    );
    assert_eq!(values.len(), 2);
    assert_eq!(values.next(), Some(2));
    consistency::assert_nodes_and_order_from_first_node_option(
        &values.it.iohm().nodes,
        &values.it.next_node,
    );
    assert_eq!(values.len(), 1);
    assert_eq!(values.next(), Some(3));
    consistency::assert_nodes_and_order_from_first_node_option(
        &values.it.iohm().nodes,
        &values.it.next_node,
    );
    assert_eq!(values.len(), 0);
    assert_eq!(values.next(), None);
    consistency::assert_nodes_and_order_from_first_node_option(
        &values.it.iohm().nodes,
        &values.it.next_node,
    );
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

    for (k, v) in iohm.iter_mut() {
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
fn test_drain_iteration() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut drain = iohm.drain();

    //println!("{}", iohm.len()); // should not compile!
    consistency::assert(&drain.it.iohm());
    assert_eq!(drain.len(), 3);
    assert_eq!(drain.next(), Some(("A", 1)));
    consistency::assert(&drain.it.iohm());
    assert_eq!(drain.len(), 2);
    assert_eq!(drain.next(), Some(("B", 2)));
    consistency::assert(&drain.it.iohm());
    assert_eq!(drain.len(), 1);
    assert_eq!(drain.next(), Some(("C", 3)));
    consistency::assert(&drain.it.iohm());
    assert_eq!(drain.len(), 0);
    assert_eq!(drain.next(), None);
    consistency::assert(&drain.it.iohm());
    mem::drop(drain);
    assert!(iohm.is_empty());
}

#[test]
fn test_drain_drop() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let mut drain = iohm.drain();

    consistency::assert(&drain.it.iohm());
    assert_eq!(drain.len(), 3);
    assert_eq!(drain.next(), Some(("A", 1)));
    consistency::assert(&drain.it.iohm());
    assert_eq!(drain.len(), 2);
    mem::drop(drain);
    assert!(iohm.is_empty());
    consistency::assert(&iohm);
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

fn assert_first_node<K, V>(iohm: &InsertionOrderHashMap<K, V>, node: &Node<K, V>) {
    let order = iohm.order.as_ref().expect("order should not be None");
    assert!(ptr::eq(order.first.as_ptr(), node));

    assert!(node.prev.is_none());
}

fn assert_last_node<K, V>(iohm: &InsertionOrderHashMap<K, V>, node: &Node<K, V>) {
    let order = iohm.order.as_ref().expect("order should not be None");
    assert!(ptr::eq(order.last.as_ptr(), node));

    assert!(node.next.is_none());
}

fn assert_linked_nodes<K, V>(node_before: &Node<K, V>, node_after: &Node<K, V>) {
    let node_before_next = node_before
        .next
        .expect("node_before.next should not be None");
    let node_before_next = unsafe { node_before_next.as_ref() };
    assert!(ptr::eq(node_before_next, node_after));

    let node_after_prev = node_after.prev.expect("node_after.prev should not be None");
    let node_after_prev = unsafe { node_after_prev.as_ref() };
    assert!(ptr::eq(node_after_prev, node_before));
}

fn as_immutable<T>(value: T) -> T {
    value
}
