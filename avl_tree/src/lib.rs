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

    use crate::Node;

    type NodeBoxOption<K, V> = Option<Box<Node<K, V>>>;

    pub(crate) fn set<K: Ord, V>(node: &mut NodeBoxOption<K, V>, key: K, value: V) -> Option<V> {
        match node {
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => {
                    let previous_value = mem::replace(&mut node.value, value);
                    Some(previous_value)
                }
                Ordering::Less => todo!(),
                Ordering::Greater => todo!(),
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
                Ordering::Less => todo!(),
                Ordering::Greater => todo!(),
            },
            None => None,
        }
    }
}
