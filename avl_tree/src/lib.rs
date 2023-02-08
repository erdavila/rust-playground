use std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub struct AVLTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> AVLTree<K, V> {
    pub fn iter(&self) -> Iter<K, V> {
        todo!()
    }
}

impl<K: Ord, V> AVLTree<K, V> {
    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        todo!()
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
