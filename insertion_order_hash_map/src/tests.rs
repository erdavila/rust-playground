use std::ptr;

use super::*;

mod entry;
mod stress_test;

#[test]
fn test_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    assert!(iohm.nodes.is_empty());
    assert!(iohm.order.is_none());
}

#[test]
fn test_get_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let iohm = as_immutable(iohm);

    let result = iohm.get(&"A");

    assert_eq!(result, Some(&1));
}

#[test]
fn test_get_non_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let iohm = as_immutable(iohm);

    let result = iohm.get(&"B");

    assert!(result.is_none());
}

#[test]
fn test_insert_on_empty() {
    let mut iohm = InsertionOrderHashMap::new();

    let result = iohm.insert("A", 1);

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 1);
    let node: &Node<_, _> = &iohm.nodes[&"A"];
    assert_eq!(unsafe { node.key.as_ref() }, &"A");
    assert_eq!(node.value, 1);
    assert_first_node(&iohm, node);
    assert_last_node(&iohm, node);
}

#[test]
fn test_insert_on_non_empty() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.insert("B", 2);

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 2);
    let node_a: &Node<_, _> = &iohm.nodes[&"A"];
    let node_b: &Node<_, _> = &iohm.nodes[&"B"];
    assert_eq!(unsafe { node_b.key.as_ref() }, &"B");
    assert_eq!(node_b.value, 2);
    assert_linked_nodes(node_a, node_b);
    assert_last_node(&iohm, node_b);
}

#[test]
fn test_insert_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.insert("A", 2);

    let iohm = as_immutable(iohm);
    assert_eq!(result, Some(1));
    assert_eq!(iohm.nodes.len(), 1);
    let node: &Node<_, _> = &iohm.nodes[&"A"];
    assert_eq!(unsafe { node.key.as_ref() }, &"A");
    assert_eq!(node.value, 2);
    assert_first_node(&iohm, node);
    assert_last_node(&iohm, node);
}

#[test]
fn test_remove_first() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.remove(&"A");

    let iohm = as_immutable(iohm);
    assert_eq!(result, Some(1));
    assert_eq!(iohm.nodes.len(), 2);
    let node: &Node<_, _> = &iohm.nodes[&"B"];
    assert_first_node(&iohm, node);
}

#[test]
fn test_remove_last() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.remove(&"C");

    let iohm = as_immutable(iohm);
    assert_eq!(result, Some(3));
    assert_eq!(iohm.nodes.len(), 2);
    let node: &Node<_, _> = &iohm.nodes[&"B"];
    assert_last_node(&iohm, node);
}

#[test]
fn test_remove_in_the_middle() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    iohm.insert("B", 2);
    iohm.insert("C", 3);

    let result = iohm.remove(&"B");

    let iohm = as_immutable(iohm);
    assert_eq!(result, Some(2));
    assert_eq!(iohm.nodes.len(), 2);
    let node_a: &Node<_, _> = &iohm.nodes[&"A"];
    let node_c: &Node<_, _> = &iohm.nodes[&"C"];
    assert_linked_nodes(node_a, node_c);
}

#[test]
fn test_remove_single_item() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.remove(&"A");

    let iohm = as_immutable(iohm);
    assert_eq!(result, Some(1));
    assert!(iohm.nodes.is_empty());
    assert!(iohm.order.is_none());
}

#[test]
fn test_remove_non_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.remove(&"B");

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 1);
}

#[test]
fn test_keys_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let keys = iohm.keys();

    let keys_vec: Vec<_> = keys.collect();
    assert!(keys_vec.is_empty());
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
