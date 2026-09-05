use std::hash::Hash;
use std::ptr;

use crate::{InsertionOrder, InsertionOrderHashMap, NodePtr, UnderlyingMap};

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
            assert_eq!(order.last, last_node.unwrap());
        }
        None => assert!(last_node.is_none()),
    }
}

pub(crate) fn assert_nodes_and_order_from_first_node_option<K, V>(
    nodes: &UnderlyingMap<K, V>,
    first_node: Option<NodePtr<K, V>>,
) -> Option<NodePtr<K, V>>
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
    first_node: NodePtr<K, V>,
) -> NodePtr<K, V>
where
    K: Hash + Eq,
{
    assert!(first_node.prev().is_none());

    let last;

    let mut count = 1usize;
    if let Some(mut current) = *first_node.next() {
        let mut previous = first_node;

        loop {
            count += 1;

            let prev_next = previous.next().unwrap();
            assert_eq!(prev_next, current);

            let curr_prev = current.prev().unwrap();
            assert_eq!(previous, curr_prev);

            let node = *nodes.get(&current.key_ptr()).unwrap();
            assert_eq!(node, current);

            if let Some(next) = current.next() {
                previous = current;
                current = *next;
            } else {
                break;
            }
        }

        last = current;
    } else {
        let node = *nodes.get(&first_node.key_ptr()).unwrap();
        assert_eq!(node, first_node);

        last = first_node;
    }

    assert_eq!(nodes.len(), count);

    for (key_wrapper, node_ptr) in nodes {
        assert!(ptr::eq(key_wrapper.0.as_ptr(), node_ptr.key()));
    }

    last
}
