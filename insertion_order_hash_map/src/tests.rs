use std::ptr;

use super::*;

mod entry;
mod stress_test;

#[test]
fn test_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    assert!(iohm.nodes.is_empty());
    assert!(iohm.order.is_none());
    assert_consistency(&iohm);
}

#[test]
fn test_get_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let iohm = as_immutable(iohm);

    let result = iohm.get(&"A");

    assert_eq!(result, Some(&1));
    assert_consistency(&iohm);
}

#[test]
fn test_get_non_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);
    let iohm = as_immutable(iohm);

    let result = iohm.get(&"B");

    assert!(result.is_none());
    assert_consistency(&iohm);
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
    assert_consistency(&iohm);
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
    assert_consistency(&iohm);
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
    assert_consistency(&iohm);
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
    let node: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"B")];
    assert_first_node(&iohm, node);
    assert_consistency(&iohm);
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
    let node: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"B")];
    assert_last_node(&iohm, node);
    assert_consistency(&iohm);
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
    let node_a: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"A")];
    let node_c: &Node<_, _> = &iohm.nodes[&KeyWrapper(&"C")];
    assert_linked_nodes(node_a, node_c);
    assert_consistency(&iohm);
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
    assert_consistency(&iohm);
}

#[test]
fn test_remove_non_existing_key() {
    let mut iohm = InsertionOrderHashMap::new();
    iohm.insert("A", 1);

    let result = iohm.remove(&"B");

    let iohm = as_immutable(iohm);
    assert!(result.is_none());
    assert_eq!(iohm.nodes.len(), 1);
    assert_consistency(&iohm);
}

#[test]
fn test_keys_on_empty() {
    let iohm: InsertionOrderHashMap<String, i32> = InsertionOrderHashMap::new();

    let keys = iohm.keys();

    let keys_vec: Vec<_> = keys.collect();
    assert!(keys_vec.is_empty());
    assert_consistency(&iohm);
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
    assert_consistency(&iohm);
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

fn assert_consistency<K: Hash + Eq, V>(iohm: &InsertionOrderHashMap<K, V>) {
    match &iohm.order {
        Some(order) => {
            fn deref<'a, T>(non_null: NonNull<T>) -> &'a T {
                unsafe { non_null.as_ref() }
            }

            let first = deref(order.first);
            assert!(first.prev.is_none());

            let last;

            let mut count = 1usize;
            if let Some(current) = first.next {
                let mut previous = first;
                let mut current = deref(current);

                loop {
                    count += 1;

                    let prev_next = previous.next.unwrap();
                    let prev_next = deref(prev_next);
                    assert!(ptr::eq(prev_next, current));

                    let curr_prev = current.prev.unwrap();
                    let curr_prev = deref(curr_prev);
                    assert!(ptr::eq(previous, curr_prev));

                    let node: &Node<K, V> = iohm.nodes.get(&KeyWrapper(&current.key)).unwrap();
                    assert!(ptr::eq(node, current));

                    if let Some(next) = current.next {
                        previous = current;
                        current = deref(next);
                    } else {
                        break;
                    }
                }
                last = current;
            } else {
                let node: &Node<K, V> = iohm.nodes.get(&KeyWrapper(&first.key)).unwrap();
                assert!(ptr::eq(node, first));

                last = first;
            }
            assert!(ptr::eq(last, deref(order.last)));

            assert_eq!(iohm.nodes.len(), count);

            for (key_wrapper, node) in &iohm.nodes {
                assert!(ptr::eq(key_wrapper.0, &node.key));
            }
        }
        None => assert!(iohm.nodes.is_empty()),
    }
}
