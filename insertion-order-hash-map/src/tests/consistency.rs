use std::hash::Hash;
use std::ptr::{self, NonNull};

use crate::{InsertionOrder, InsertionOrderHashMap, KeyWrapper, Node, UnderlyingMap};

pub fn assert<K, V>(iohm: &InsertionOrderHashMap<K, V>)
where
    K: Hash + Eq,
{
    assert_nodes_and_order(&iohm.nodes, iohm.order.as_ref());
}

pub(crate) fn assert_nodes_and_order<K, V>(
    nodes: &UnderlyingMap<K, V>,
    order: Option<&InsertionOrder<K, V>>,
) where
    K: Hash + Eq,
{
    let last_node = assert_nodes_and_order_from_first_node_option(
        nodes,
        order.as_ref().map(|order| order.first),
    );

    match order {
        Some(order) => {
            assert!(ptr::eq(order.last.as_ptr(), last_node.unwrap()));
        }
        None => assert!(last_node.is_none()),
    }
}

pub(crate) fn assert_nodes_and_order_from_first_node_option<K, V>(
    nodes: &UnderlyingMap<K, V>,
    first_node: Option<NonNull<Node<K, V>>>,
) -> Option<&Node<K, V>>
where
    K: Hash + Eq,
{
    if let Some(first_node) = first_node {
        let last_node = assert_nodes_and_order_from_first_node(nodes, first_node);
        Some(last_node)
    } else {
        assert!(nodes.is_empty());
        None
    }
}

pub(crate) fn assert_nodes_and_order_from_first_node<K, V>(
    nodes: &UnderlyingMap<K, V>,
    first_node: NonNull<Node<K, V>>,
) -> &Node<K, V>
where
    K: Hash + Eq,
{
    let first: &Node<K, V> = deref(first_node);
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

            let node: &Node<K, V> = nodes.get(&KeyWrapper(&raw const current.key)).unwrap();
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
        let node: &Node<K, V> = nodes.get(&KeyWrapper(&raw const first.key)).unwrap();
        assert!(ptr::eq(node, first));

        last = first;
    }

    assert_eq!(nodes.len(), count);

    for (key_wrapper, node) in nodes {
        assert!(ptr::eq(key_wrapper.0, &raw const node.key));
    }

    last
}

fn deref<'a, T>(non_null: NonNull<T>) -> &'a T {
    unsafe { non_null.as_ref() }
}
