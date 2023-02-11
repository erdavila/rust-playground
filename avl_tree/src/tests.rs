use crate::AVLTree;

mod balancing;

#[test]
fn test_set_on_empty_tree() {
    let mut tree = AVLTree::new();

    let previous_value = tree.set("A", 1);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 1);
}

#[test]
fn test_get_non_existing_key() {
    let tree = AVLTree::<&str, i32>::new();

    let value = tree.get(&"X");

    assert!(value.is_none());
}

#[test]
fn test_set_replacing() {
    let mut tree = AVLTree::new();
    tree.set("A", -1);

    let previous_value = tree.set("A", 1);

    assert_eq!(previous_value, Some(-1));
    assert_eq!(tree.get(&"A"), Some(&1));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 1);
}

#[test]
fn test_set_adding_left_child() {
    let mut tree = AVLTree::new();
    tree.set("B", 2);

    let previous_value = tree.set("A", 1);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 2);
}

#[test]
fn test_set_adding_right_child() {
    let mut tree = AVLTree::new();
    tree.set("A", 1);

    let previous_value = tree.set("B", 2);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 2);
}

#[test]
fn test_set_with_right_rotation() {
    let mut tree = AVLTree::new();
    tree.set("C", 3);
    tree.set("B", 2);

    let previous_value = tree.set("A", 1);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    assert_eq!(tree.get(&"C"), Some(&3));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_set_with_left_rotation() {
    let mut tree = AVLTree::new();
    tree.set("A", 1);
    tree.set("B", 2);

    let previous_value = tree.set("C", 3);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    assert_eq!(tree.get(&"C"), Some(&3));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_set_with_left_right_double_rotation() {
    let mut tree = AVLTree::new();
    tree.set("C", 3);
    tree.set("A", 1);

    let previous_value = tree.set("B", 2);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    assert_eq!(tree.get(&"C"), Some(&3));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_set_with_right_left_double_rotation() {
    let mut tree = AVLTree::new();
    tree.set("A", 1);
    tree.set("C", 3);

    let previous_value = tree.set("B", 2);

    assert!(previous_value.is_none());
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    assert_eq!(tree.get(&"C"), Some(&3));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_unset_non_existing_element() {
    let mut tree = AVLTree::<&str, i32>::new();

    let value = tree.unset(&"A");

    assert!(value.is_none());
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 0);
}

#[test]
fn test_unset_single_node() {
    let mut tree = AVLTree::new();
    tree.set("A", 1);

    let value = tree.unset(&"A");

    assert_eq!(value, Some(("A", 1)));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 0);
}

#[test]
fn test_unset_promoting_left_child() {
    let mut tree = AVLTree::new();
    tree.set("B", 2);
    tree.set("A", 1);

    let value = tree.unset(&"B");

    assert_eq!(value, Some(("B", 2)));
    assert_eq!(tree.get(&"A"), Some(&1));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 1);
}

#[test]
fn test_unset_promoting_right_child() {
    let mut tree = AVLTree::new();
    tree.set("A", 1);
    tree.set("B", 2);

    let value = tree.unset(&"A");

    assert_eq!(value, Some(("A", 1)));
    assert_eq!(tree.get(&"B"), Some(&2));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 1);
}

#[test]
fn test_unset_root_no_rebalancing_1() {
    let mut tree = AVLTree::new();
    tree.set("C", 3);
    tree.set("B", 2);
    tree.set("D", 4);
    tree.set("A", 1);

    let value = tree.unset(&"C");

    assert_eq!(value, Some(("C", 3)));
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    assert_eq!(tree.get(&"D"), Some(&4));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_unset_root_no_rebalancing_2() {
    let mut tree = AVLTree::new();
    tree.set("C", 3);
    tree.set("A", 1);
    tree.set("D", 4);
    tree.set("B", 2);

    let value = tree.unset(&"C");

    assert_eq!(value, Some(("C", 3)));
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"B"), Some(&2));
    assert_eq!(tree.get(&"D"), Some(&4));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_unset_root_no_rebalancing_3() {
    let mut tree = AVLTree::new();
    tree.set("B", 2);
    tree.set("A", 1);
    tree.set("D", 4);
    tree.set("C", 3);

    let value = tree.unset(&"B");

    assert_eq!(value, Some(("B", 2)));
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"C"), Some(&3));
    assert_eq!(tree.get(&"D"), Some(&4));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}

#[test]
fn test_unset_root_no_rebalancing_4() {
    let mut tree = AVLTree::new();
    tree.set("B", 2);
    tree.set("A", 1);
    tree.set("C", 3);
    tree.set("D", 4);

    let value = tree.unset(&"B");

    assert_eq!(value, Some(("B", 2)));
    assert_eq!(tree.get(&"A"), Some(&1));
    assert_eq!(tree.get(&"C"), Some(&3));
    assert_eq!(tree.get(&"D"), Some(&4));
    let evaluation = balancing::assert_on_tree(&tree);
    assert_eq!(evaluation.node_count, 3);
}
