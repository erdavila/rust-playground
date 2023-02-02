use std::collections::HashMap;
use std::collections::TryReserveError;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

#[cfg(test)]
mod tests;

type UnderlyingMap<K, V> = HashMap<KeyWrapper<K>, Box<Node<K, V>>>;

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

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.order = None;
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
        let self_ref = unsafe { &mut *(self as *mut Self) };

        match self.nodes.get_mut(&KeyWrapper(&key)) {
            Some(node) => Entry::Occupied(OccupiedEntry {
                node,
                iohm: self_ref,
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                iohm: self_ref,
            }),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.nodes.get(&KeyWrapper(key)).map(|node| &node.value)
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.nodes
            .get(&KeyWrapper(key))
            .map(|node| (&node.key, &node.value))
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.nodes.contains_key(&KeyWrapper(key))
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.nodes
            .get_mut(&KeyWrapper(key))
            .map(|node| &mut node.value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.nodes.get_mut(&KeyWrapper(&key)) {
            Some(node) => {
                let previous_value = mem::replace(&mut node.as_mut().value, value);
                Some(previous_value)
            }
            None => {
                let vacant_entry = VacantEntry { key, iohm: self };
                vacant_entry.insert(value);

                None
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.remove_entry(key).map(|(_, value)| value)
    }

    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        match self.nodes.remove(&KeyWrapper(key)) {
            Some(mut node) => {
                node.unlink(&mut self.order);
                Some((node.key, node.value))
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

#[derive(Eq)]
struct KeyWrapper<T>(*const T);
impl<T> KeyWrapper<T> {
    fn get_ref(&self) -> &T {
        unsafe { &*self.0 }
    }
}
impl<T: Hash> Hash for KeyWrapper<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_ref().hash(state);
    }
}
impl<T: PartialEq> PartialEq for KeyWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_ref() == other.get_ref()
    }
}

struct Node<K, V> {
    key: K,
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
impl<'a, K, V: 'a> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next_node {
            Some(node) => {
                let key = unsafe { &node.as_ref().key };
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
impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq,
{
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
                Self::Occupied(OccupiedEntry { ..occupied_entry })
            }
            Self::Vacant(vacant_entry) => Self::Vacant(VacantEntry { ..vacant_entry }),
        }
    }
}
impl<'a, K, V: Default> Entry<'a, K, V>
where
    K: Hash + Eq,
{
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
    node: &'a mut Node<K, V>,
    iohm: &'a mut InsertionOrderHashMap<K, V>,
}
impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        &self.node.key
    }

    pub fn remove_entry(self) -> (K, V)
    where
        K: Hash + Eq,
    {
        let node = self.iohm.nodes.remove(&KeyWrapper(&self.node.key)).unwrap();
        self.node.unlink(&mut self.iohm.order);

        (node.key, node.value)
    }

    pub fn get(&self) -> &V {
        &self.node.value
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.node.value
    }

    pub fn into_mut(self) -> &'a mut V {
        &mut self.node.value
    }

    pub fn insert(&mut self, value: V) -> V {
        mem::replace(&mut self.node.value, value)
    }

    pub fn remove(self) -> V
    where
        K: Hash + Eq,
    {
        self.remove_entry().1
    }
}

pub struct VacantEntry<'a, K, V> {
    key: K,
    iohm: &'a mut InsertionOrderHashMap<K, V>,
}
impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Eq + Hash,
    {
        let node = Box::new(Node {
            key: self.key,
            value,
            prev: None,
            next: None,
        });

        let entry = self.iohm.nodes.entry(KeyWrapper(&node.key));
        let node = entry.or_insert(node);
        node.link(&mut self.iohm.order);

        &mut node.value
    }
}
