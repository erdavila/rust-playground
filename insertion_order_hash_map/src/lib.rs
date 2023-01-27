use std::collections::TryReserveError;
use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

#[cfg(test)]
mod tests;

type UnderlyingMap<K, V> = HashMap<Box<K>, Box<Node<K, V>>>;

pub struct InsertionOrderHashMap<K, V> {
    nodes: UnderlyingMap<K, V>,
    order: Option<InsertionOrder<K, V>>,
}
impl<K, V> InsertionOrderHashMap<K, V> {
    pub fn new() -> Self {
        Self::with_underlying_map(Default::default())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_underlying_map(HashMap::with_capacity(capacity))
    }

    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    fn with_underlying_map(nodes: UnderlyingMap<K, V>) -> Self {
        InsertionOrderHashMap { nodes, order: None }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Hash + Eq,
    {
        self.nodes.get(key).map(|node| &node.value)
    }

    pub fn set(&mut self, key: K, value: V) -> Option<V>
    where
        K: Hash + Eq,
    {
        match self.nodes.entry(Box::new(key)) {
            Entry::Occupied(mut occupied) => {
                let previous_value = mem::replace(&mut occupied.get_mut().value, value);
                Some(previous_value)
            }
            Entry::Vacant(vacant) => {
                let node = Box::new(Node {
                    key: NonNull::from(vacant.key().as_ref()),
                    value,
                    prev: None,
                    next: None,
                });

                let node = vacant.insert(node);

                if let Some(order) = &mut self.order {
                    node.prev = Some(order.last);
                    unsafe { order.last.as_mut() }.next = Some(NonNull::from(node.as_ref()));
                    order.last = NonNull::from(node.as_ref());
                } else {
                    self.order = Some(InsertionOrder {
                        first: NonNull::from(node.as_ref()),
                        last: NonNull::from(node.as_ref()),
                    });
                }

                None
            }
        }
    }

    pub fn unset(&mut self, key: &K) -> Option<V>
    where
        K: Hash + Eq,
    {
        match self.nodes.remove(key) {
            Some(node) => {
                match (node.prev, node.next) {
                    (Some(mut prev), Some(mut next)) => {
                        unsafe { prev.as_mut() }.next = Some(next);
                        unsafe { next.as_mut() }.prev = Some(prev);
                    }
                    (Some(mut prev), None) => {
                        unsafe { prev.as_mut() }.next = None;
                        self.order.as_mut().unwrap().last = prev;
                    }
                    (None, Some(mut next)) => {
                        unsafe { next.as_mut() }.prev = None;
                        self.order.as_mut().unwrap().first = next;
                    }
                    (None, None) => self.order = None,
                }
                Some(node.value)
            }
            None => None,
        }
    }

    pub fn keys(&self) -> Keys<K, V> {
        Keys {
            next_node: self.order.as_ref().map(|order| order.first),
            phantom: PhantomData,
        }
    }
}
impl<K, V> InsertionOrderHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.nodes.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.nodes.shrink_to(min_capacity);
    }
}
impl<K, V> Default for InsertionOrderHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

struct Node<K, V> {
    key: NonNull<K>,
    value: V,
    prev: Option<NonNull<Node<K, V>>>,
    next: Option<NonNull<Node<K, V>>>,
}

struct InsertionOrder<K, V> {
    first: NonNull<Node<K, V>>,
    last: NonNull<Node<K, V>>,
}

pub struct Keys<'a, K: 'a, V> {
    next_node: Option<NonNull<Node<K, V>>>,
    phantom: PhantomData<&'a K>,
}
impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next_node {
            Some(node) => {
                let key = unsafe { node.as_ref().key.as_ref() };
                self.next_node = unsafe { node.as_ref() }.next;
                Some(key)
            }
            None => None,
        }
    }
}
