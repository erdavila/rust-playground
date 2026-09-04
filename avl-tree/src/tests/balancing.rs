use std::fmt::Debug;

use crate::{AVLTree, Height, Node};

pub struct TreeEvaluation {
    pub height: Height,
    pub node_count: usize,
}

impl TreeEvaluation {
    fn zeroes() -> Self {
        TreeEvaluation {
            height: 0,
            node_count: 0,
        }
    }
}

pub fn assert_on_tree<K: Ord + Debug, V>(avl_tree: &AVLTree<K, V>) -> TreeEvaluation {
    if let Some(ref root) = avl_tree.root {
        assert_on_node(root, None, None)
    } else {
        TreeEvaluation::zeroes()
    }
}

fn assert_on_node<K: Ord + Debug, V>(
    node: &Node<K, V>,
    min_key: Option<&K>,
    max_key: Option<&K>,
) -> TreeEvaluation {
    if let Some(min_key) = min_key {
        assert!(node.key > *min_key);
    }
    if let Some(max_key) = max_key {
        assert!(node.key < *max_key);
    }

    let left_node_evaluation = match node.left {
        Some(ref left) => assert_on_node(left, min_key, Some(&node.key)),
        None => TreeEvaluation::zeroes(),
    };
    let right_node_evaluation = match node.right {
        Some(ref right) => assert_on_node(right, Some(&node.key), max_key),
        None => TreeEvaluation::zeroes(),
    };

    let diff = i16::from(left_node_evaluation.height) - i16::from(right_node_evaluation.height);
    assert!(diff >= -1);
    assert!(diff <= 1);

    let node_height = 1 + left_node_evaluation
        .height
        .max(right_node_evaluation.height);
    assert_eq!(node.height, node_height, "node with key {:?}", node.key);

    TreeEvaluation {
        height: node_height,
        node_count: 1 + left_node_evaluation.node_count + right_node_evaluation.node_count,
    }
}
