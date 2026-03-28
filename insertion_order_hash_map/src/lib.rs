use std::borrow::Borrow;
use std::collections::{HashMap, TryReserveError};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::mem;
use std::ops::Index;
use std::ptr::{NonNull, from_mut};

#[cfg(test)]
mod tests;

type UnderlyingMap<K, V> = HashMap<KeyWrapper<K>, Box<Node<K, V>>>;

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
        self.visiting_iterator(|node| &node.key)
    }

    #[must_use]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.consuming_iterator(|node| node.key)
    }

    #[must_use]
    pub fn values(&self) -> Values<'_, K, V> {
        self.visiting_iterator(|node| &node.value)
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.visiting_iterator_mut(|node| &mut node.value)
    }

    #[must_use]
    pub fn into_values(self) -> IntoValues<K, V> {
        self.consuming_iterator(|node| node.value)
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.visiting_iterator(|node| (&node.key, &node.value))
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.visiting_iterator_mut(|node| (&node.key, &mut node.value))
    }

    fn visiting_iterator<'a, O>(
        &'a self,
        visit: VisitingFunction<'a, K, V, O>,
    ) -> VisitingIterator<'a, K, V, O> {
        VisitingIterator {
            next_node: self.order.as_ref().map(|order| order.first),
            visit,
            len: self.nodes.len(),
        }
    }

    fn visiting_iterator_mut<'a, O>(
        &'a self,
        visit: VisitingFunctionMut<'a, K, V, O>,
    ) -> VisitingIteratorMut<'a, K, V, O> {
        VisitingIteratorMut {
            next_node: self.order.as_ref().map(|order| order.first),
            visit,
            len: self.nodes.len(),
        }
    }

    fn consuming_iterator<O>(
        self,
        consume: ConsumingFunction<K, V, O>,
    ) -> ConsumingIterator<K, V, O> {
        let mut iohm = Box::new(self);
        let iohm_ref: &mut InsertionOrderHashMap<_, _> = unsafe { mem::transmute(&mut *iohm) };

        ConsumingIterator {
            iohm,
            it: InternalConsumingIterator::new(iohm_ref, consume),
        }
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
        Drain {
            it: InternalConsumingIterator::new(self, |node| (node.key, node.value)),
            phantom: PhantomData,
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
        K: Hash + Eq,
    {
        let mut node_option = self.order.as_ref().map(|order| order.first);
        while let Some(mut node) = node_option {
            let node = unsafe { node.as_mut() };

            let keep = f(&node.key, &mut node.value);

            node_option = node.next;

            if !keep {
                self.remove_node(&node.key);
            }
        }
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
        let self_ref = unsafe { &mut *from_mut::<Self>(self) };

        match self.nodes.get_mut(&KeyWrapper(&raw const key)) {
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

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = BorrowWrapper::from_ref(k);
        self.nodes.get(q).map(|node| &node.value)
    }

    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = BorrowWrapper::from_ref(k);
        self.nodes.get(q).map(|node| (&node.key, &node.value))
    }

    #[must_use]
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        self.end_node_key_value(|order| order.first)
    }

    pub fn first_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>> {
        self.end_node_entry(|order| order.first)
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.pop_end_node(|order| order.first)
    }

    #[must_use]
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        self.end_node_key_value(|order| order.last)
    }

    pub fn last_entry(&mut self) -> Option<OccupiedEntry<'_, K, V>> {
        self.end_node_entry(|order| order.last)
    }

    pub fn pop_last(&mut self) -> Option<(K, V)> {
        self.pop_end_node(|order| order.last)
    }

    fn end_node_key_value(&self, get_end_node: GetEndNode<K, V>) -> Option<(&K, &V)> {
        self.order.as_ref().map(|order| {
            let node = get_end_node(order);
            let node = unsafe { node.as_ref() };
            (&node.key, &node.value)
        })
    }

    fn end_node_entry(
        &mut self,
        get_end_node: GetEndNode<K, V>,
    ) -> Option<OccupiedEntry<'_, K, V>> {
        let self_ref = unsafe { &mut *from_mut::<Self>(self) };
        self.order.as_mut().map(|order| {
            let mut node = get_end_node(order);
            let node = unsafe { node.as_mut() };
            OccupiedEntry {
                node,
                iohm: self_ref,
            }
        })
    }

    fn pop_end_node(&mut self, get_end_node: GetEndNode<K, V>) -> Option<(K, V)> {
        match &self.order {
            Some(order) => {
                let node = get_end_node(order);
                let node = unsafe { node.as_ref() };
                let node = self.remove_node(&node.key);
                Some((node.key, node.value))
            }
            None => None,
        }
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = BorrowWrapper::from_ref(k);
        self.nodes.contains_key(q)
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let q = BorrowWrapper::from_ref(k);
        self.nodes.get_mut(q).map(|node| &mut node.value)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(node) = self.nodes.get_mut(&KeyWrapper(&raw const key)) {
            let previous_value = node.replace_value(value);
            Some(previous_value)
        } else {
            let vacant_entry = VacantEntry { key, iohm: self };
            vacant_entry.insert(value);

            None
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
        let q = BorrowWrapper::from_ref(k);
        let self_ref = unsafe { &mut *from_mut::<Self>(self) };
        match self.nodes.get_mut(q) {
            Some(node) => {
                let occupied_entry = OccupiedEntry {
                    node: node.as_mut(),
                    iohm: self_ref,
                };
                let key_value = occupied_entry.remove_entry();
                Some(key_value)
            }
            None => None,
        }
    }

    fn remove_node(&mut self, key: &K) -> Node<K, V> {
        let mut node = self.nodes.remove(&KeyWrapper(key)).unwrap();
        node.unlink(&mut self.order);
        *node
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        self.visiting_iterator(|node| (&node.key, &node.value))
    }
}
impl<'a, K, V> IntoIterator for &'a mut InsertionOrderHashMap<K, V> {
    type Item = (&'a K, &'a mut V);

    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.visiting_iterator_mut(|node| (&node.key, &mut node.value))
    }
}
impl<K: Hash + Eq, V> IntoIterator for InsertionOrderHashMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.consuming_iterator(|node| (node.key, node.value))
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

    fn replace_value(&mut self, value: V) -> V {
        mem::replace(&mut self.value, value)
    }
}

type GetEndNode<K, V> = fn(&InsertionOrder<K, V>) -> NonNull<Node<K, V>>;

struct InsertionOrder<K, V> {
    first: NonNull<Node<K, V>>,
    last: NonNull<Node<K, V>>,
}

type VisitingFunction<'a, K, V, O> = fn(&'a Node<K, V>) -> O;
pub struct VisitingIterator<'a, K, V, O> {
    next_node: Option<NonNull<Node<K, V>>>,
    visit: VisitingFunction<'a, K, V, O>,
    len: usize,
}
impl<'a, K, V, O: 'a> Iterator for VisitingIterator<'a, K, V, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next_node {
            Some(node) => {
                let node = unsafe { node.as_ref() };
                self.next_node = node.next;
                self.len -= 1;

                let output = (self.visit)(node);
                Some(output)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a, K, V, O: 'a> ExactSizeIterator for VisitingIterator<'a, K, V, O> {
    fn len(&self) -> usize {
        self.len
    }
}
impl<'a, K, V, O: 'a> FusedIterator for VisitingIterator<'a, K, V, O> {}

pub type Keys<'a, K, V> = VisitingIterator<'a, K, V, &'a K>;
pub type Values<'a, K, V> = VisitingIterator<'a, K, V, &'a V>;
pub type Iter<'a, K, V> = VisitingIterator<'a, K, V, (&'a K, &'a V)>;

type VisitingFunctionMut<'a, K, V, O> = fn(&'a mut Node<K, V>) -> O;
pub struct VisitingIteratorMut<'a, K, V, O> {
    next_node: Option<NonNull<Node<K, V>>>,
    visit: VisitingFunctionMut<'a, K, V, O>,
    len: usize,
}
impl<'a, K, V, O: 'a> Iterator for VisitingIteratorMut<'a, K, V, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.next_node {
            &Some(mut node) => {
                let node = unsafe { node.as_mut() };
                self.next_node = node.next;
                self.len -= 1;

                let output = (self.visit)(node);
                Some(output)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<'a, K, V, O: 'a> ExactSizeIterator for VisitingIteratorMut<'a, K, V, O> {
    fn len(&self) -> usize {
        self.len
    }
}
impl<'a, K, V, O: 'a> FusedIterator for VisitingIteratorMut<'a, K, V, O> {}

pub type ValuesMut<'a, K, V> = VisitingIteratorMut<'a, K, V, &'a mut V>;
pub type IterMut<'a, K, V> = VisitingIteratorMut<'a, K, V, (&'a K, &'a mut V)>;

type ConsumingFunction<K, V, O> = fn(Node<K, V>) -> O;
struct InternalConsumingIterator<K, V, O> {
    next_node: Option<NonNull<Node<K, V>>>,
    iohm: NonNull<InsertionOrderHashMap<K, V>>,
    consume: ConsumingFunction<K, V, O>,
}
impl<K, V, O> InternalConsumingIterator<K, V, O> {
    fn new(iohm: &mut InsertionOrderHashMap<K, V>, consume: ConsumingFunction<K, V, O>) -> Self {
        InternalConsumingIterator {
            next_node: iohm.order.as_ref().map(|order| order.first),
            iohm: NonNull::from(iohm),
            consume,
        }
    }

    fn iohm(&self) -> &InsertionOrderHashMap<K, V> {
        unsafe { self.iohm.as_ref() }
    }

    fn iohm_mut(&mut self) -> &mut InsertionOrderHashMap<K, V> {
        unsafe { self.iohm.as_mut() }
    }
}
impl<K: Hash + Eq, V, O> Iterator for InternalConsumingIterator<K, V, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_node {
            Some(mut node) => {
                let node = unsafe { node.as_mut() };
                let node = self.iohm_mut().remove_node(&node.key);

                self.next_node = node.next;

                Some((self.consume)(node))
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<K: Hash + Eq, V, O> ExactSizeIterator for InternalConsumingIterator<K, V, O> {
    fn len(&self) -> usize {
        self.iohm().len()
    }
}
impl<K: Hash + Eq, V, O> FusedIterator for InternalConsumingIterator<K, V, O> {}

pub struct ConsumingIterator<K, V, O> {
    #[expect(dead_code)]
    iohm: Box<InsertionOrderHashMap<K, V>>,
    it: InternalConsumingIterator<K, V, O>,
}
impl<K: Hash + Eq, V, O> Iterator for ConsumingIterator<K, V, O> {
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
impl<K: Hash + Eq, V, O> ExactSizeIterator for ConsumingIterator<K, V, O> {
    fn len(&self) -> usize {
        self.it.len()
    }
}
impl<K: Hash + Eq, V, O> FusedIterator for ConsumingIterator<K, V, O> {}

pub type IntoKeys<K, V> = ConsumingIterator<K, V, K>;
pub type IntoValues<K, V> = ConsumingIterator<K, V, V>;
pub type IntoIter<K, V> = ConsumingIterator<K, V, (K, V)>;

pub struct Drain<'a, K, V> {
    it: InternalConsumingIterator<K, V, (K, V)>,
    phantom: PhantomData<&'a ()>,
}
impl<K: Hash + Eq, V> Iterator for Drain<'_, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
impl<K, V> Drop for Drain<'_, K, V> {
    fn drop(&mut self) {
        self.it.iohm_mut().clear();
    }
}
impl<K: Hash + Eq, V> ExactSizeIterator for Drain<'_, K, V> {
    fn len(&self) -> usize {
        self.it.len()
    }
}
impl<K: Hash + Eq, V> FusedIterator for Drain<'_, K, V> {}

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
                let ptr = from_mut::<V>(occupied_entry.get_mut());
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

    #[must_use]
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
                let ptr = from_mut::<V>(occupied_entry.get_mut());
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
    #[must_use]
    pub fn key(&self) -> &K {
        &self.node.key
    }

    #[must_use]
    pub fn remove_entry(self) -> (K, V)
    where
        K: Hash + Eq,
    {
        let node = self.iohm.remove_node(&self.node.key);
        (node.key, node.value)
    }

    #[must_use]
    pub fn get(&self) -> &V {
        &self.node.value
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.node.value
    }

    #[must_use]
    pub fn into_mut(self) -> &'a mut V {
        &mut self.node.value
    }

    pub fn insert(&mut self, value: V) -> V {
        self.node.replace_value(value)
    }

    #[must_use]
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

        let entry = self.iohm.nodes.entry(KeyWrapper(&raw const node.key));
        let node = entry.or_insert(node);
        node.link(&mut self.iohm.order);

        &mut node.value
    }
}

#[derive(Hash, PartialEq, Eq)]
#[repr(transparent)]
struct BorrowWrapper<T: ?Sized>(T);
impl<T: ?Sized> BorrowWrapper<T> {
    fn from_ref(r: &T) -> &Self {
        unsafe { &*(std::ptr::from_ref::<T>(r) as *const BorrowWrapper<T>) }
    }
}

impl<T, Q> Borrow<BorrowWrapper<Q>> for KeyWrapper<T>
where
    T: Borrow<Q>,
    Q: ?Sized,
{
    fn borrow(&self) -> &BorrowWrapper<Q> {
        BorrowWrapper::from_ref(self.get_ref().borrow())
    }
}
