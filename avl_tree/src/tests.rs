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
