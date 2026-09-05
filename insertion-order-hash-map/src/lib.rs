#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::boxed::Box;
use core::borrow::Borrow;
use core::convert::identity;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem;
use core::ops::Index;
use core::ptr::NonNull;

use hashbrown::{Equivalent, HashMap, TryReserveError};

#[cfg(test)]
mod tests;

type UnderlyingMap<K, V> = HashMap<KeyPtr<K>, NodePtr<K, V>>;

pub struct InsertionOrderHashMap<K, V> {
    nodes: UnderlyingMap<K, V>,
    order: Option<InsertionOrder<K, V>>,
}
impl<K, V> InsertionOrderHashMap<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Self::with_underlying_map(HashMap::default())
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_underlying_map(HashMap::with_capacity(capacity))
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    fn with_underlying_map(nodes: UnderlyingMap<K, V>) -> Self {
        InsertionOrderHashMap { nodes, order: None }
    }

    #[must_use]
    pub fn keys(&self) -> Keys<'_, K, V> {
        GeneralIterator::new(self, NodePtr::key)
    }

    #[must_use]
    pub fn into_keys(mut self) -> IntoKeys<K, V> {
        GeneralIterator::new_consuming(&mut self, NodePtr::into_key)
    }

    #[must_use]
    pub fn values(&self) -> Values<'_, K, V> {
        GeneralIterator::new(self, NodePtr::value)
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        GeneralIterator::new(self, NodePtr::value_mut)
    }

    #[must_use]
    pub fn into_values(mut self) -> IntoValues<K, V> {
        GeneralIterator::new_consuming(&mut self, NodePtr::into_value)
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        GeneralIterator::new(self, NodePtr::key_value)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        GeneralIterator::new(self, NodePtr::key_value_mut)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn drain(&mut self) -> Drain<'_, K, V> {
        Drain::new(self)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
        K: Hash + Eq,
    {
        for node_ptr in GeneralIterator::new(self, identity) {
            let keep = f(node_ptr.key(), node_ptr.value_mut());

            if !keep {
                self.nodes.remove(&node_ptr.key_ptr());
                let node = node_ptr.unlink(&mut self.order);
                drop(node);
            }
        }
    }

    pub fn clear(&mut self) {
        self.drain();
    }
}
impl<K, V> InsertionOrderHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    #[expect(clippy::missing_errors_doc)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.nodes.try_reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.nodes.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.nodes.shrink_to(min_capacity);
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.nodes.get_mut(&KeyPtr::new(&key)) {
            Some(&mut node_ptr) => Entry::Occupied(OccupiedEntry {
                node_ptr,
                iohm: self,
            }),
            None => Entry::Vacant(VacantEntry { key, iohm: self }),
        }
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = QueryKey(k);
        self.nodes.get(&q).map(|node_ptr| node_ptr.value())
    }

    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = QueryKey(k);
        self.nodes
            .get(&q)
            .map(|node_ptr| (node_ptr.key(), node_ptr.value()))
    }

    #[must_use]
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        self.end_node_key_value(|order| order.first)
    }

    pub fn first_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>> {
        self.end_node_entry(|order| order.first)
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.pop_end_key_value(|order| order.first)
    }

    #[must_use]
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        self.end_node_key_value(|order| order.last)
    }

    pub fn last_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>> {
        self.end_node_entry(|order| order.last)
    }

    pub fn pop_last(&mut self) -> Option<(K, V)> {
        self.pop_end_key_value(|order| order.last)
    }

    fn end_node_key_value(&self, get_end_node: GetEndNode<K, V>) -> Option<(&K, &V)> {
        self.order.as_ref().map(|order| {
            let node_ptr = get_end_node(order);
            (node_ptr.key(), node_ptr.value())
        })
    }

    fn end_node_entry(
        &mut self,
        get_end_node: GetEndNode<K, V>,
    ) -> Option<OccupiedEntry<'_, K, V>> {
        let order = self.order.as_ref()?;
        let node_ptr = get_end_node(order);
        Some(OccupiedEntry {
            node_ptr,
            iohm: self,
        })
    }

    fn pop_end_key_value(&mut self, get_end_node: GetEndNode<K, V>) -> Option<(K, V)> {
        let order = self.order.as_ref()?;

        let node_ptr = get_end_node(order);
        self.nodes.remove(&node_ptr.key_ptr());
        let node = node_ptr.unlink(&mut self.order);

        (self.order.is_some()).then_some((node.key, node.value))
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = QueryKey(k);
        self.nodes.contains_key(&q)
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = QueryKey(k);
        self.nodes.get_mut(&q).map(|node_ptr| node_ptr.value_mut())
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.entry(key) {
            Entry::Occupied(mut occupied_entry) => {
                let previous_value = mem::replace(occupied_entry.get_mut(), value);
                Some(previous_value)
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(value);
                None
            }
        }
    }

    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.remove_entry(k).map(|(_, value)| value)
    }

    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = QueryKey(k);
        let node_ptr = self.nodes.remove(&q)?;
        let node = node_ptr.unlink(&mut self.order);
        Some((node.key, node.value))
    }
}
impl<K, V> Drop for InsertionOrderHashMap<K, V> {
    fn drop(&mut self) {
        let it = GeneralIterator::new(self, NodePtr::into_node);
        drop(it);
    }
}
impl<K, V> Clone for InsertionOrderHashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let cloned = self.iter().map(|(k, v)| (k.clone(), v.clone()));
        cloned.collect()
    }
}
impl<K, V> Debug for InsertionOrderHashMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("InsertionOrderHashMap ")?;
        f.debug_map().entries(self).finish()
    }
}
impl<K, V> Default for InsertionOrderHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a, K, V> Extend<(&'a K, &'a V)> for InsertionOrderHashMap<K, V>
where
    K: Eq + Hash + Copy,
    V: Copy,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(*k, *v);
        }
    }
}
impl<K, V> Extend<(K, V)> for InsertionOrderHashMap<K, V>
where
    K: Eq + Hash,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}
impl<K, V, const N: usize> From<[(K, V); N]> for InsertionOrderHashMap<K, V>
where
    K: Eq + Hash,
{
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}
impl<K, V> FromIterator<(K, V)> for InsertionOrderHashMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut iohm = InsertionOrderHashMap::new();
        iohm.extend(iter);
        iohm
    }
}
impl<K, Q, V> Index<&Q> for InsertionOrderHashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}
impl<'a, K, V> IntoIterator for &'a InsertionOrderHashMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, K, V> IntoIterator for &'a mut InsertionOrderHashMap<K, V> {
    type Item = (&'a K, &'a mut V);

    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl<K, V> IntoIterator for InsertionOrderHashMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<K, V>;

    fn into_iter(mut self) -> Self::IntoIter {
        GeneralIterator::new_consuming(&mut self, NodePtr::into_key_value)
    }
}

#[derive(Debug, Eq)]
struct KeyPtr<K: ?Sized>(NonNull<K>);
impl<K: ?Sized> KeyPtr<K> {
    fn new(key: &K) -> Self {
        KeyPtr(NonNull::from_ref(key))
    }

    fn get_ref(&self) -> &K {
        unsafe { &*self.0.as_ptr() }
    }
}
impl<K: Hash> Hash for KeyPtr<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ref().hash(state);
    }
}
impl<K: PartialEq + ?Sized> PartialEq for KeyPtr<K> {
    fn eq(&self, other: &Self) -> bool {
        self.get_ref() == other.get_ref()
    }
}

#[derive(Hash)]
struct QueryKey<'a, Q: ?Sized>(&'a Q);
impl<K, Q> Equivalent<KeyPtr<K>> for QueryKey<'_, Q>
where
    K: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn equivalent(&self, key: &KeyPtr<K>) -> bool {
        self.0 == key.get_ref().borrow()
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<NodePtr<K, V>>,
    next: Option<NodePtr<K, V>>,
}

struct NodePtr<K, V>(NonNull<Node<K, V>>);
impl<K, V> NodePtr<K, V> {
    fn key_ptr(self) -> KeyPtr<K> {
        KeyPtr::new(self.key())
    }

    fn key<'a>(self) -> &'a K {
        unsafe { &(*self.0.as_ptr()).key }
    }

    fn into_key(self) -> K {
        self.into_node().key
    }

    fn value<'a>(self) -> &'a V {
        unsafe { &(*self.0.as_ptr()).value }
    }

    fn value_mut<'a>(self) -> &'a mut V {
        unsafe { &mut (*self.0.as_ptr()).value }
    }

    fn into_value(self) -> V {
        self.into_node().value
    }

    fn replace_value(self, value: V) -> V {
        mem::replace(self.value_mut(), value)
    }

    fn key_value<'a>(self) -> (&'a K, &'a V) {
        (self.key(), self.value())
    }

    fn key_value_mut<'a>(self) -> (&'a K, &'a mut V) {
        (self.key(), self.value_mut())
    }

    fn into_key_value(self) -> (K, V) {
        let node = self.into_node();
        (node.key, node.value)
    }

    #[expect(clippy::ref_option)]
    fn prev(&self) -> &Option<NodePtr<K, V>> {
        unsafe { &(*self.0.as_ptr()).prev }
    }

    fn prev_mut(&mut self) -> &mut Option<NodePtr<K, V>> {
        unsafe { &mut (*self.0.as_ptr()).prev }
    }

    #[expect(clippy::ref_option)]
    fn next(&self) -> &Option<NodePtr<K, V>> {
        unsafe { &(*self.0.as_ptr()).next }
    }

    fn next_mut(&mut self) -> &mut Option<NodePtr<K, V>> {
        unsafe { &mut (*self.0.as_ptr()).next }
    }

    fn link_last(&mut self, order_opt: &mut Option<InsertionOrder<K, V>>) {
        if let Some(order) = order_opt {
            *self.prev_mut() = Some(order.last);
            *order.last.next_mut() = Some(*self);
            order.last = *self;
        } else {
            *order_opt = Some(InsertionOrder {
                first: *self,
                last: *self,
            });
        }
    }

    #[must_use]
    fn unlink(self, order_opt: &mut Option<InsertionOrder<K, V>>) -> Node<K, V> {
        let order = order_opt.as_mut().unwrap();

        match (*self.prev(), *self.next()) {
            (Some(mut prev), Some(mut next)) => {
                *prev.next_mut() = Some(next);
                *next.prev_mut() = Some(prev);
            }
            (Some(mut prev), None) => {
                *prev.next_mut() = None;
                order.last = prev;
            }
            (None, Some(mut next)) => {
                *next.prev_mut() = None;
                order.first = next;
            }
            (None, None) => {
                *order_opt = None;
            }
        }

        self.into_node()
    }

    fn into_node(self) -> Node<K, V> {
        let boxed = unsafe { Box::from_raw(self.0.as_ptr()) };
        *boxed
    }
}
impl<K, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<K, V> Copy for NodePtr<K, V> {}
impl<K, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K, V> Eq for NodePtr<K, V> {}
impl<K, V> From<Node<K, V>> for NodePtr<K, V> {
    fn from(node: Node<K, V>) -> Self {
        NodePtr(NonNull::new(Box::into_raw(Box::new(node))).unwrap())
    }
}
impl<K, V> Debug for NodePtr<K, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NodePtrWrapper").field(&self.0).finish()
    }
}

type GetEndNode<K, V> = fn(&InsertionOrder<K, V>) -> NodePtr<K, V>;

#[derive(Debug)]
struct InsertionOrder<K, V> {
    first: NodePtr<K, V>,
    last: NodePtr<K, V>,
}
impl<K, V> Clone for InsertionOrder<K, V> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<K, V> Copy for InsertionOrder<K, V> {}

pub struct GeneralIterator<K, V, O> {
    order: Option<InsertionOrder<K, V>>,
    len: usize,
    f: fn(NodePtr<K, V>) -> O,
}
impl<K, V, O> GeneralIterator<K, V, O> {
    fn new(iohm: &InsertionOrderHashMap<K, V>, f: fn(NodePtr<K, V>) -> O) -> Self {
        GeneralIterator {
            order: iohm.order,
            len: iohm.nodes.len(),
            f,
        }
    }

    fn new_consuming(iohm: &mut InsertionOrderHashMap<K, V>, f: fn(NodePtr<K, V>) -> O) -> Self {
        let this = Self::new(iohm, f);
        iohm.nodes.clear();
        iohm.order = None;
        this
    }
}
impl<K, V, O> Iterator for GeneralIterator<K, V, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.order.as_mut()?;
        let node_ptr = inner.first;

        if let Some(next) = node_ptr.next() {
            inner.first = *next;
        } else {
            self.order = None;
        }

        self.len -= 1;
        Some((self.f)(node_ptr))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<K, V, O> DoubleEndedIterator for GeneralIterator<K, V, O> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.order.as_mut()?;
        let node_ptr = inner.last;

        if let Some(prev) = node_ptr.prev() {
            inner.last = *prev;
        } else {
            self.order = None;
        }

        self.len -= 1;
        Some((self.f)(node_ptr))
    }
}
impl<K, V, O> ExactSizeIterator for GeneralIterator<K, V, O> {
    fn len(&self) -> usize {
        self.len
    }
}
impl<K, V, O> FusedIterator for GeneralIterator<K, V, O> {}
impl<K, V, O> Drop for GeneralIterator<K, V, O> {
    fn drop(&mut self) {
        while self.next().is_some() {}
    }
}

pub type Keys<'a, K, V> = GeneralIterator<K, V, &'a K>;
pub type Values<'a, K, V> = GeneralIterator<K, V, &'a V>;
pub type Iter<'a, K, V> = GeneralIterator<K, V, (&'a K, &'a V)>;
pub type ValuesMut<'a, K, V> = GeneralIterator<K, V, &'a mut V>;
pub type IterMut<'a, K, V> = GeneralIterator<K, V, (&'a K, &'a mut V)>;
pub type IntoKeys<K, V> = GeneralIterator<K, V, K>;
pub type IntoValues<K, V> = GeneralIterator<K, V, V>;
pub type IntoIter<K, V> = GeneralIterator<K, V, (K, V)>;

pub struct Drain<'a, K, V> {
    it: GeneralIterator<K, V, (K, V)>,
    phantom: PhantomData<&'a ()>,
}
impl<K, V> Drain<'_, K, V> {
    fn new(iohm: &mut InsertionOrderHashMap<K, V>) -> Self {
        Drain {
            it: GeneralIterator::new_consuming(iohm, NodePtr::into_key_value),
            phantom: PhantomData,
        }
    }
}
impl<K, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
impl<K, V> DoubleEndedIterator for Drain<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back()
    }
}
impl<K, V> ExactSizeIterator for Drain<'_, K, V> {
    fn len(&self) -> usize {
        self.it.len()
    }
}
impl<K, V> FusedIterator for Drain<'_, K, V> {}

#[derive(Debug)]
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
            Self::Occupied(occupied_entry) => occupied_entry.node_ptr.value_mut(),
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

    #[must_use]
    pub fn and_modify<F: FnOnce(&mut V)>(mut self, f: F) -> Self {
        if let Self::Occupied(occupied_entry) = &mut self {
            let value = occupied_entry.get_mut();
            f(value);
        }

        self
    }
}
impl<'a, K, V: Default> Entry<'a, K, V>
where
    K: Hash + Eq,
{
    pub fn or_default(self) -> &'a mut V {
        match self {
            Self::Occupied(occupied_entry) => occupied_entry.node_ptr.value_mut(),
            Self::Vacant(vacant_entry) => vacant_entry.insert(V::default()),
        }
    }
}

#[derive(Debug)]
pub struct OccupiedEntry<'a, K, V> {
    node_ptr: NodePtr<K, V>,
    iohm: &'a mut InsertionOrderHashMap<K, V>,
}
impl<'a, K, V> OccupiedEntry<'a, K, V> {
    #[must_use]
    pub fn key(&self) -> &K {
        self.node_ptr.key()
    }

    #[must_use]
    pub fn remove_entry(self) -> (K, V)
    where
        K: Hash + Eq,
    {
        self.iohm.nodes.remove_entry(&self.node_ptr.key_ptr());
        let node = self.node_ptr.unlink(&mut self.iohm.order);
        (node.key, node.value)
    }

    #[must_use]
    pub fn get(&self) -> &V {
        self.node_ptr.value()
    }

    pub fn get_mut(&mut self) -> &mut V {
        self.node_ptr.value_mut()
    }

    #[must_use]
    pub fn into_mut(self) -> &'a mut V {
        self.node_ptr.value_mut()
    }

    pub fn insert(&mut self, value: V) -> V {
        self.node_ptr.replace_value(value)
    }

    #[must_use]
    pub fn remove(self) -> V
    where
        K: Hash + Eq,
    {
        self.remove_entry().1
    }
}

#[derive(Debug)]
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
        let mut node_ptr = NodePtr::from(Node {
            key: self.key,
            value,
            prev: None,
            next: None,
        });

        let key_ptr = node_ptr.key_ptr();

        unsafe {
            self.iohm.nodes.insert_unique_unchecked(key_ptr, node_ptr);
        }

        node_ptr.link_last(&mut self.iohm.order);

        node_ptr.value_mut()
    }
}
