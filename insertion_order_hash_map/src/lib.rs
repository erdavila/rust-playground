use std::collections::TryReserveError;
use std::collections::{hash_map, HashMap};
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

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        match self.nodes.entry(Box::new(key)) {
            hash_map::Entry::Occupied(underlying_occupied_entry) => {
                Entry::Occupied(OccupiedEntry {
                    underlying_occupied_entry,
                    order: &mut self.order,
                })
            }
            hash_map::Entry::Vacant(underlying_vacant_entry) => Entry::Vacant(VacantEntry {
                underlying_vacant_entry,
                order: &mut self.order,
            }),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.nodes.get(key).map(|node| &node.value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.nodes.entry(Box::new(key)) {
            hash_map::Entry::Occupied(mut occupied) => {
                let previous_value = mem::replace(&mut occupied.get_mut().value, value);
                Some(previous_value)
            }
            hash_map::Entry::Vacant(vacant) => {
                let node = Box::new(Node {
                    key: NonNull::from(vacant.key().as_ref()),
                    value,
                    prev: None,
                    next: None,
                });

                let node = vacant.insert(node);
                node.link(&mut self.order);

                None
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.nodes.remove(key) {
            Some(mut node) => {
                node.unlink(&mut self.order);
                Some(node.value)
            }
            None => None,
        }
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
impl<K, V> Node<K, V> {
    fn link(&mut self, order: &mut Option<InsertionOrder<K, V>>) {
        if let Some(order) = order {
            self.prev = Some(order.last);
            unsafe { order.last.as_mut() }.next = Some(NonNull::from(&*self));
            order.last = NonNull::from(&*self);
        } else {
            *order = Some(InsertionOrder {
                first: NonNull::from(&*self),
                last: NonNull::from(&*self),
            });
        }
    }

    fn unlink(&mut self, order: &mut Option<InsertionOrder<K, V>>) {
        match (self.prev, self.next) {
            (Some(mut prev), Some(mut next)) => {
                unsafe { prev.as_mut() }.next = Some(next);
                unsafe { next.as_mut() }.prev = Some(prev);
            }
            (Some(mut prev), None) => {
                unsafe { prev.as_mut() }.next = None;
                order.as_mut().unwrap().last = prev;
            }
            (None, Some(mut next)) => {
                unsafe { next.as_mut() }.prev = None;
                order.as_mut().unwrap().first = next;
            }
            (None, None) => {
                *order = None;
            }
        }
    }
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

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
impl<'a, K, V> Entry<'a, K, V> {
    pub fn or_insert(self, default: V) -> &'a mut V {
        self.or_insert_with(|| default)
    }

    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        self.or_insert_with_key(|_| default())
    }

    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Self::Occupied(mut occupied_entry) => {
                let ptr = occupied_entry.get_mut() as *mut V;
                unsafe { &mut *ptr }
            }
            Self::Vacant(vacant_entry) => {
                let value = default(vacant_entry.key());
                vacant_entry.insert(value)
            }
        }
    }

    pub fn key(&self) -> &K {
        match self {
            Self::Occupied(occupied_entry) => occupied_entry.key(),
            Self::Vacant(vacant_entry) => vacant_entry.key(),
        }
    }

    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        match self {
            Self::Occupied(mut occupied_entry) => {
                let value = occupied_entry.get_mut();
                f(value);
                Self::Occupied(OccupiedEntry {
                    underlying_occupied_entry: occupied_entry.underlying_occupied_entry,
                    order: occupied_entry.order,
                })
            }
            Self::Vacant(vacant_entry) => Self::Vacant(VacantEntry {
                underlying_vacant_entry: vacant_entry.underlying_vacant_entry,
                order: vacant_entry.order,
            }),
        }
    }
}
impl<'a, K, V: Default> Entry<'a, K, V> {
    pub fn or_default(self) -> &'a mut V {
        match self {
            Self::Occupied(mut occupied_entry) => {
                let ptr = occupied_entry.get_mut() as *mut V;
                unsafe { &mut *ptr }
            }
            Self::Vacant(vacant_entry) => vacant_entry.insert(V::default()),
        }
    }
}

pub struct OccupiedEntry<'a, K, V> {
    underlying_occupied_entry: hash_map::OccupiedEntry<'a, Box<K>, Box<Node<K, V>>>,
    order: &'a mut Option<InsertionOrder<K, V>>,
}
impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        let node = self.underlying_occupied_entry.get();
        let key_ptr = node.key;
        unsafe { key_ptr.as_ref() }
    }

    pub fn remove_entry(self) -> (K, V) {
        let (key, mut node) = self.underlying_occupied_entry.remove_entry();
        node.unlink(self.order);

        (*key, node.value)
    }

    pub fn get(&self) -> &V {
        let node = self.underlying_occupied_entry.get();
        &node.value
    }

    pub fn get_mut(&mut self) -> &mut V {
        let node = self.underlying_occupied_entry.get_mut();
        &mut node.value
    }

    pub fn into_mut(self) -> &'a mut V {
        let node = self.underlying_occupied_entry.into_mut();
        &mut node.value
    }

    pub fn insert(&mut self, value: V) -> V {
        let node = self.underlying_occupied_entry.get_mut();
        mem::replace(&mut node.value, value)
    }

    pub fn remove(self) -> V {
        self.remove_entry().1
    }
}

pub struct VacantEntry<'a, K, V> {
    underlying_vacant_entry: hash_map::VacantEntry<'a, Box<K>, Box<Node<K, V>>>,
    order: &'a mut Option<InsertionOrder<K, V>>,
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        self.underlying_vacant_entry.key()
    }

    pub fn into_key(self) -> K {
        *self.underlying_vacant_entry.into_key()
    }

    pub fn insert(self, value: V) -> &'a mut V {
        let key: &K = self.underlying_vacant_entry.key();

        let node = Box::new(Node {
            key: NonNull::from(key),
            value,
            prev: None,
            next: None,
        });

        let node = self.underlying_vacant_entry.insert(node);
        node.link(self.order);

        &mut node.value
    }
}
