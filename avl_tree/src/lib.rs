use std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub struct AVLTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> AVLTree<K, V> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn iter(&self) -> Iter<K, V> {
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

    pub fn unset(&self, key: &K) -> Option<(K, V)> {
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

                    let left_height = height(&node.left);
                    let right_height = height(&node.right);

                    if left_height - right_height > 1 {
                        /*
                         *        self-> O:N
                         *                |
                         *        node-> B:N
                         *                |
                         *                N
                         *              /   \
                         *          O:L       O:R
                         *           |
                         *          B:L
                         *           |
                         *           L
                         *         /  \
                         *     O:LL    O:P
                         */
                        let mut left = node.left.take().unwrap();
                        /*
                         *                        self-> O:N
                         *                                |
                         *    left= B:L           node-> B:N
                         *           |                    |
                         *           L                    N
                         *         /   \                /   \
                         *     O:LL     O:P      O:None       O:R
                         */
                        let pivot = left.right.take();
                        /*
                         *                                         self-> O:N
                         *                                                 |
                         *    left= B:L                            node-> B:N
                         *           |                                     |
                         *           L             pivot= O:P              N
                         *         /   \                                 /   \
                         *     O:LL     O:None                    O:None       O:R
                         */
                        let pivot_height = height(&pivot);
                        if pivot_height > height(&left.left) {
                            // Perform left right double rotation
                            /*
                             *              N                         N
                             *           /     \                    /   \                   P
                             *       L           R                P       R               /   \
                             *     /   \              =>         / \           =>      L         N
                             *   LL      P                    L     PR               /  \       /  \
                             *          / \                 /  \                   LL    PL   PR     R
                             *        PL   PR             LL    PL
                             */
                            /*
                             *                         pivot= O:P           self-> O:N
                             *                                 |                    |
                             *    left= B:L                   B:P           node-> B:N
                             *           |                     |                    |
                             *           L                     P                    N
                             *         /   \                 /   \                /   \
                             *     O:LL     O:None        O:PL   O:PR      O:None       O:R
                             */
                            let mut pivot = pivot.unwrap();
                            /*
                             *                                              self-> O:N
                             *                                                      |
                             *    left= B:L            pivot= B:P           node-> B:N
                             *           |                     |                    |
                             *           L                     P                    N
                             *         /   \                 /   \                /   \
                             *     O:LL     O:None        O:PL   O:PR      O:None       O:R
                             */
                            left.right = pivot.left.take();
                            left.height = 1 + height(&left.left).max(height(&left.right));
                            /*
                             *                                              self-> O:N
                             *                                                      |
                             *    left= B:L            pivot= B:P           node-> B:N
                             *           |                     |                    |
                             *           L                     P                    N
                             *         /   \                 /   \                /   \
                             *     O:LL     O:PL        O:None   O:PR      O:None       O:R
                             */
                            pivot.left = Some(left);
                            /*
                             *      pivot= B:P           self-> O:N
                             *              |                    |
                             *              P            node-> B:N
                             *            /   \                  |
                             *          O:L   O:PR               N
                             *           |                     /   \
                             *          B:L             O:None       O:R
                             *           |
                             *           L
                             *         /   \
                             *     O:LL     O:PL
                             */
                            node.left = pivot.right.take();
                            node.height = 1 + height(&node.left).max(height(&node.right));
                            /*
                             *      pivot= B:P           self-> O:N
                             *              |                    |
                             *              P            node-> B:N
                             *            /   \                  |
                             *          O:L    O:None            N
                             *           |                     /   \
                             *          B:L               O:PR       O:R
                             *           |
                             *           L
                             *         /   \
                             *     O:LL     O:PL
                             */
                            let right = mem::replace(node, pivot);
                            /*
                             *      self-> O:P
                             *              |
                             *      node-> B:P
                             *              |
                             *              P            right= B:N
                             *            /   \                  |
                             *          O:L    O:None            N
                             *           |                     /   \
                             *          B:L               O:PR       O:R
                             *           |
                             *           L
                             *         /   \
                             *     O:LL     O:PL
                             */
                            node.right = Some(right);
                            node.height = 1 + height(&node.left).max(height(&node.right));
                            /*
                             *          self-> O:P
                             *                  |
                             *          node-> B:P
                             *                  |
                             *                  P
                             *               /     \
                             *          O:L           O:N
                             *           |             |
                             *          B:L           B:N
                             *           |             |
                             *           L             N
                             *         /   \         /   \
                             *     O:LL    O:PL   O:PR     O:R
                             */

                            result
                        } else {
                            // Perform right rotation
                            node.left = pivot;
                            node.height = 1 + pivot_height.max(right_height);
                            /*
                             *                        self-> O:N
                             *                                |
                             *    left= B:L           node-> B:N
                             *           |                    |
                             *           L                    N
                             *         /   \                /   \
                             *     O:LL     O:None      O:P       O:R
                             */
                            let right = mem::replace(node, left);
                            /*
                             *   self-> O:N
                             *           |
                             *   node-> B:L           right= B:N
                             *           |                    |
                             *           L                    N
                             *         /   \                /   \
                             *     O:LL     O:None      O:P       O:R
                             */
                            node.right = Some(right);
                            /*
                             *   self-> O:N
                             *           |
                             *   node-> B:L
                             *           |
                             *           L
                             *         /   \
                             *     O:LL      O:N
                             *                |
                             *               B:N
                             *                |
                             *                N
                             *              /   \
                             *          O:P       O:R
                             */

                            result
                        }
                    } else {
                        node.height = 1 + right_height.max(left_height);

                        result
                    }
                }
                Ordering::Greater => {
                    let result = set(&mut node.right, key, value);

                    let right_height = height(&node.right);
                    let left_height = height(&node.left);

                    if right_height - left_height > 1 {
                        let mut right = node.right.take().unwrap();
                        let pivot_height = height(&right.left);
                        if pivot_height > height(&right.right) {
                            todo!("double rotation")
                        } else {
                            // Perform left rotation
                            node.right = right.left.take();
                            node.height = 1 + pivot_height.max(left_height);
                            let left = mem::replace(node, right);
                            node.left = Some(left);

                            result
                        }
                    } else {
                        node.height = 1 + left_height.max(right_height);

                        result
                    }
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

    pub(crate) fn height<K, V>(node: &NodeBoxOption<K, V>) -> Height {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }
}
