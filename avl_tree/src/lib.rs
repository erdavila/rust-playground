use std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub type AVLTreeMap<K, V> = AVLTree<K, V>;
pub type AVLTreeSet<E> = AVLTree<E, PhantomData<()>>;

pub struct AVLTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> AVLTree<K, V> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn new_map() -> Self {
        Self::new()
    }

    pub fn iter(&self) -> Iter<K, V> {
        todo!()
    }

    pub fn breadth_iter(&self) -> BreadthIter<K, V> {
        todo!()
    }
}

impl<K: Ord, V> AVLTree<K, V> {
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        algo::set(&mut self.root, key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        algo::get(&self.root, key)
    }

    pub fn unset(&mut self, key: &K) -> Option<(K, V)> {
        algo::unset(&mut self.root, key)
    }

    pub fn contains(&self, key: &K) -> bool {
        todo!()
    }
}

impl<E> AVLTreeSet<E> {
    pub fn new_set() -> Self {
        Self::new()
    }
}

impl<E: Ord> AVLTreeSet<E> {
    pub fn add(&mut self, element: E) -> bool {
        todo!()
    }

    pub fn remove(&mut self, element: &E) -> bool {
        todo!()
    }
}

impl<K, V> IntoIterator for AVLTree<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<K, V> Default for AVLTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    height: Height,
}

impl<K, V> Node<K, V> {
    fn update_height(&mut self) {
        self.height = 1 + algo::height(&self.left).max(algo::height(&self.right));
    }
}

type Height = u8;

pub struct Iter<'a, K, V> {
    phantom: PhantomData<(K, V, &'a ())>,
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct BreadthIter<'a, K, V> {
    phantom: PhantomData<(K, V, &'a ())>,
}

impl<'a, K: 'a, V: 'a> Iterator for BreadthIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IntoIter<K, V> {
    phantom: PhantomData<(K, V)>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

mod algo {
    use std::cmp::Ordering;
    use std::mem;

    use crate::{Height, Node};

    type NodeBoxOption<K, V> = Option<Box<Node<K, V>>>;

    pub(crate) fn set<K: Ord, V>(node: &mut NodeBoxOption<K, V>, key: K, value: V) -> Option<V> {
        match node {
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => {
                    let previous_value = mem::replace(&mut node.value, value);
                    Some(previous_value)
                }
                Ordering::Less => {
                    let result = set(&mut node.left, key, value);

                    if height(&node.left) - height(&node.right) > 1 {
                        let mut left = node.left.take().unwrap();

                        if height(&left.right) > height(&left.left) {
                            let pivot = left.right.take();
                            rotate_left(&mut left, pivot.unwrap());
                        }

                        rotate_right(node, left);
                    } else {
                        node.update_height();
                    }

                    result
                }
                Ordering::Greater => {
                    let result = set(&mut node.right, key, value);

                    if height(&node.right) - height(&node.left) > 1 {
                        let mut right = node.right.take().unwrap();

                        if height(&right.left) > height(&right.right) {
                            let pivot = right.left.take();
                            rotate_right(&mut right, pivot.unwrap());
                        }

                        rotate_left(node, right);
                    } else {
                        node.update_height();
                    }

                    result
                }
            },
            None => {
                *node = Some(Box::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                    height: 1,
                }));

                None
            }
        }
    }

    pub(crate) fn get<'a, K: Ord, V>(node: &'a NodeBoxOption<K, V>, key: &K) -> Option<&'a V> {
        match node {
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => Some(&node.value),
                Ordering::Less => get(&node.left, key),
                Ordering::Greater => get(&node.right, key),
            },
            None => None,
        }
    }

    pub(crate) fn unset<K: Ord, V>(
        node_option: &mut NodeBoxOption<K, V>,
        key: &K,
    ) -> Option<(K, V)> {
        match node_option {
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => match (&node.left, &node.right) {
                    (Some(left), Some(right)) => {
                        if left.height > right.height {
                            let (replacing_key, replacing_value) = unset_max(&mut node.left);
                            node.update_height();

                            let removed_key = mem::replace(&mut node.key, replacing_key);
                            let removed_value = mem::replace(&mut node.value, replacing_value);

                            if height(&node.right) - height(&node.left) > 1 {
                                todo!()
                            }

                            Some((removed_key, removed_value))
                        } else {
                            let (replacing_key, replacing_value) = unset_min(&mut node.right);
                            node.update_height();

                            let removed_key = mem::replace(&mut node.key, replacing_key);
                            let removed_value = mem::replace(&mut node.value, replacing_value);

                            if height(&node.left) - height(&node.right) > 1 {
                                todo!()
                            }

                            Some((removed_key, removed_value))
                        }
                    }
                    (None, _) => {
                        let right = node.right.take();
                        let node = mem::replace(node_option, right).unwrap();
                        Some((node.key, node.value))
                    }
                    (_, None) => {
                        let left = node.left.take();
                        let node = mem::replace(node_option, left).unwrap();
                        Some((node.key, node.value))
                    }
                },
                Ordering::Less => {
                    let removed_key_value = unset(&mut node.left, key);

                    if removed_key_value.is_some() {
                        if height(&node.right) - height(&node.left) > 1 {
                            todo!()
                        } else {
                            node.update_height();
                        }
                    }

                    removed_key_value
                }
                Ordering::Greater => {
                    let removed_key_value = unset(&mut node.right, key);

                    if removed_key_value.is_some() {
                        if height(&node.left) - height(&node.right) > 1 {
                            todo!()
                        } else {
                            node.update_height();
                        }
                    }

                    removed_key_value
                }
            },
            None => None,
        }
    }

    fn unset_max<K, V>(node_option: &mut NodeBoxOption<K, V>) -> (K, V) {
        let node = node_option.as_deref_mut().unwrap();
        match node.right {
            Some(_) => {
                let key_value = unset_max(&mut node.right);

                if height(&node.left) - height(&node.right) > 1 {
                    todo!()
                } else {
                    node.update_height();
                }

                key_value
            }
            None => {
                let left_option = node.left.take();
                let node = mem::replace(node_option, left_option).unwrap();
                (node.key, node.value)
            }
        }
    }

    fn unset_min<K, V>(node_option: &mut NodeBoxOption<K, V>) -> (K, V) {
        let node = node_option.as_deref_mut().unwrap();
        match node.left {
            Some(_) => {
                let key_value = unset_max(&mut node.left);

                if height(&node.right) - height(&node.left) > 1 {
                    todo!()
                } else {
                    node.update_height();
                }

                key_value
            }
            None => {
                let right_option = node.right.take();
                let node = mem::replace(node_option, right_option).unwrap();
                (node.key, node.value)
            }
        }
    }

    pub(crate) fn height<K, V>(node: &NodeBoxOption<K, V>) -> Height {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }

    fn rotate_right<K, V>(node: &mut Box<Node<K, V>>, left: Box<Node<K, V>>) {
        rotate(node, left, RotationDirection::Right);
    }

    fn rotate_left<K, V>(node: &mut Box<Node<K, V>>, right: Box<Node<K, V>>) {
        rotate(node, right, RotationDirection::Left);
    }

    fn rotate<K, V>(
        node: &mut Box<Node<K, V>>,
        mut opposite_direction_child: Box<Node<K, V>>,
        direction: RotationDirection,
    ) {
        /*
         * These diagrams show what happens on a LEFT rotation (direction = RotationDirection::Left)
         *
         *
         *       N                      R
         *     /   \                  /   \
         *   L       R      =>      N      RR
         *          /  \          /  \
         *         P    RR       L    P
         */
        debug_assert!(node.child(direction.opposite()).is_none());
        /*
         * node-> B:N
         *       /   \
         *     O:L    O:None
         *                  `-X-\
         *                        right= B:R
         *                              /   \
         *                            O:P   O:RR
         */
        let pivot = opposite_direction_child.child_mut(direction).take();
        /*
         * node-> B:N                            right= B:R
         *       /   \          pivot= O:P             /   \
         *     O:L    O:None                     O:None    O:RR
         */
        *node.child_mut(direction.opposite()) = pivot;
        node.update_height();
        /*
         * node-> B:N           right= B:R
         *       /   \                /   \
         *     O:L   O:P        O:None    O:RR
         */
        let direction_child = mem::replace(node, opposite_direction_child);
        /*
         * right= B:N           node-> B:R
         *       /   \                /   \
         *     O:L   O:P        O:None    O:RR
         */
        *node.child_mut(direction) = Some(direction_child);
        node.update_height();
        /*
         *         node-> B:R
         *              /     \
         *          O:N         O:RR
         *           |
         *    left= B:N
         *         /   \
         *      O:L     O:P
         */
    }

    #[derive(Clone, Copy)]
    enum RotationDirection {
        Left,
        Right,
    }

    impl RotationDirection {
        fn opposite(&self) -> RotationDirection {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
            }
        }
    }

    impl<K, V> Node<K, V> {
        fn child(&self, rotation_direction: RotationDirection) -> &NodeBoxOption<K, V> {
            match rotation_direction {
                RotationDirection::Left => &self.left,
                RotationDirection::Right => &self.right,
            }
        }

        fn child_mut(&mut self, rotation_direction: RotationDirection) -> &mut NodeBoxOption<K, V> {
            match rotation_direction {
                RotationDirection::Left => &mut self.left,
                RotationDirection::Right => &mut self.right,
            }
        }
    }
}
