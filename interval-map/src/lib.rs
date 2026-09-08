#![no_std]
// #![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod interval;
// TODO: pub mod iter;
mod insert;
pub mod relative;
mod values_between_exist;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem;

use intrusive_collections::{Bound, KeyAdapter, RBTree, RBTreeLink, intrusive_adapter};
pub use values_between_exist::*;

use crate::insert::Insert;
use crate::interval::{Endpoint, Interval, IntervalLike};
use crate::relative::Relative;

/*
    TODO: use `&Q` where `K: Borrow<Q>` for queries.
*/

pub struct IntervalMap<K, V> {
    tree: RBTree<NodeAdapter<K, V>>,
}

impl<K, V> IntervalMap<K, V> {
    #[must_use]
    pub fn new() -> Self {
        IntervalMap {
            tree: RBTree::default(),
        }
    }
}

impl<K, V> IntervalMap<K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    /*
        TODO: Return iterator.
            - Inserts and removes nodes during iteration.
            - When dropped, removes remaining node without cloning for `previous` entries.
        TODO: Test returned entries.
    */
    pub fn insert(&mut self, interval: impl IntervalLike<K>, value: V) -> Insert<'_, K, V>
    // TODO: remove bounds
    where
        K: Debug,
        V: Debug,
    {
        let inserting = IntervalAndValue {
            interval: interval.into_interval(),
            value,
        };

        Insert::new(self, inserting)
    }
}

impl<K, V> IntervalMap<K, V>
where
    K: Ord + 'static,
{
    pub fn get(&self, key: &K) -> Option<&V> {
        // self.entry(key).map(|entry| entry.get())

        Self::do_get(
            key,
            |bound| self.tree.upper_bound(bound).get(),
            |node| &node.value,
        )
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        // self.entry_mut(key).map(|mut entry| entry.get_mut())

        Self::do_get(
            key,
            |bound| {
                self.tree
                    .upper_bound_mut(bound)
                    .get_ptr()
                    .map(|mut node_ptr| {
                        // SAFETY:
                        // 1. `&mut self` guarantees exclusive access to the map and all its nodes.
                        // 2. The returned reference's lifetime is tied to `&mut self`.
                        // 3. We only project into `value`, leaving `interval` and `link` untouched,
                        //    so RBTree ordering and tree structure invariants are preserved.
                        unsafe { node_ptr.as_mut() }
                    })
            },
            |node| &mut node.value,
        )
    }

    fn do_get<N: Borrow<Node<K, V>>, R>(
        key: &K,
        get_node: impl FnOnce(Bound<&Relative<&K>>) -> Option<N>,
        value_ref: fn(N) -> R,
    ) -> Option<R> {
        get_node(Bound::Included(&Relative::At(key))).and_then(|node| {
            (node.borrow().interval.right_relative() >= key).then(|| value_ref(node))
        })
    }
}

impl<K, V> IntervalMap<K, V> {
    // TODO: return iterator
    pub fn remove(&mut self, interval: impl IntervalLike<K>) -> Vec<(Interval<K>, V)> {
        todo!()
    }

    pub fn entry(&self, key: &K) -> Option<Entry<'_, K, V>> {
        todo!()
    }

    pub fn entry_mut(&mut self, key: &K) -> Option<EntryMut<'_, K, V>> {
        todo!()
    }

    pub fn entry_at_or_after(&self, key: &K) -> Option<Entry<'_, K, V>> {
        todo!()
    }

    pub fn entry_at_or_after_mut(&mut self, key: &K) -> Option<EntryMut<'_, K, V>> {
        todo!()
    }

    pub fn entry_at_or_before(&self, key: &K) -> Option<Entry<'_, K, V>> {
        todo!()
    }

    pub fn entry_at_or_before_mut(&mut self, key: &K) -> Option<EntryMut<'_, K, V>> {
        todo!()
    }

    pub fn first_entry(&self) -> Option<Entry<'_, K, V>> {
        todo!()
    }

    pub fn first_entry_mut(&mut self) -> Option<EntryMut<'_, K, V>> {
        todo!()
    }

    pub fn last_entry(&self) -> Option<Entry<'_, K, V>> {
        todo!()
    }

    pub fn last_entry_mut(&mut self) -> Option<EntryMut<'_, K, V>> {
        todo!()
    }

    // #[must_use]
    // pub fn iter(&self) -> Iter<'_, K, V> {
    //     todo!()
    // }

    // pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
    //     todo!()
    // }

    // pub fn iter_containing(&self, interval: impl IntervalLike<K>) -> IterContaining<'_, K, V> {
    //     todo!()
    // }

    // pub fn iter_containing_mut(
    //     &mut self,
    //     interval: impl IntervalLike<K>,
    // ) -> IterContainingMut<'_, K, V> {
    //     todo!()
    // }
}

impl<K, V> Clone for IntervalMap<K, V> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<K, V> PartialEq for IntervalMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<K, V> Eq for IntervalMap<K, V> {}

impl<K, V> Default for IntervalMap<K, V> {
    fn default() -> Self {
        IntervalMap::new()
    }
}

impl<K: Debug, V: Debug> Debug for IntervalMap<K, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map()
            .entries(self.tree.iter().map(|node| (&node.interval, &node.value)))
            .finish()
    }
}

// impl<K, V> IntoIterator for IntervalMap<K, V> {
//     type Item = <Self::IntoIter as Iterator>::Item;

//     type IntoIter = IntoIter<K, V>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

// impl<'a, K, V> IntoIterator for &'a IntervalMap<K, V> {
//     type Item = <Self::IntoIter as Iterator>::Item;

//     type IntoIter = Iter<'a, K, V>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

// impl<'a, K, V> IntoIterator for &'a mut IntervalMap<K, V> {
//     type Item = <Self::IntoIter as Iterator>::Item;

//     type IntoIter = IterMut<'a, K, V>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }

struct IntervalAndValue<K, V> {
    interval: Interval<K>,
    value: V,
}
impl<K, V> IntervalAndValue<K, V> {
    fn left(&self) -> &Endpoint<K> {
        &self.interval.left
    }

    fn left_mut(&mut self) -> &mut Endpoint<K> {
        &mut self.interval.left
    }

    fn right(&self) -> &Endpoint<K> {
        &self.interval.right
    }

    fn right_mut(&mut self) -> &mut Endpoint<K> {
        &mut self.interval.right
    }

    fn left_relative(&self) -> Relative<&K> {
        self.interval.left_relative()
    }

    fn right_relative(&self) -> Relative<&K> {
        self.interval.right_relative()
    }

    fn into_tuple(self) -> (Interval<K>, V) {
        (self.interval, self.value)
    }
}
// TODO: needed?
impl<K, V> From<(Interval<K>, V)> for IntervalAndValue<K, V> {
    fn from((interval, value): (Interval<K>, V)) -> Self {
        IntervalAndValue { interval, value }
    }
}
impl<K: Debug, V: Debug> Debug for IntervalAndValue<K, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.interval.fmt(f)?;
        f.write_str(" -> ")?;
        self.value.fmt(f)?;
        Ok(())
    }
}

struct Node<K, V> {
    interval: Interval<K>,
    value: V,
    link: RBTreeLink,
}
impl<K, V> Node<K, V> {
    // TODO: may be `IntervalAndValue<K, V>` instead of `impl Into<IntervalAndValue<K, V>>`?
    fn new(interval_and_value: impl Into<IntervalAndValue<K, V>>) -> Box<Self> {
        let IntervalAndValue { interval, value } = interval_and_value.into();
        Box::new(Node {
            interval,
            value,
            link: RBTreeLink::new(),
        })
    }

    fn left(&self) -> &Endpoint<K> {
        &self.interval.left
    }

    fn right(&self) -> &Endpoint<K> {
        &self.interval.right
    }

    fn left_relative(&self) -> Relative<&K> {
        self.interval.left_relative()
    }

    fn right_relative(&self) -> Relative<&K> {
        self.interval.right_relative()
    }

    fn replace(
        &mut self,
        IntervalAndValue { interval, value }: IntervalAndValue<K, V>,
    ) -> (Interval<K>, V) {
        let interval = self.replace_interval(interval);
        let value = self.replace_value(value);
        (interval, value)
    }

    fn replace_interval(&mut self, interval: Interval<K>) -> Interval<K> {
        mem::replace(&mut self.interval, interval)
    }

    fn replace_left(&mut self, left: Endpoint<K>) -> Endpoint<K> {
        mem::replace(&mut self.interval.left, left)
    }

    fn replace_right(&mut self, right: Endpoint<K>) -> Endpoint<K> {
        mem::replace(&mut self.interval.right, right)
    }

    fn replace_value(&mut self, value: V) -> V {
        mem::replace(&mut self.value, value)
    }

    fn swap_left(&mut self, left: &mut Endpoint<K>) {
        mem::swap(&mut self.interval.left, left);
    }

    fn swap_right(&mut self, right: &mut Endpoint<K>) {
        mem::swap(&mut self.interval.right, right);
    }
}

intrusive_adapter!(NodeAdapter<K, V> = Box<Node<K, V>>: Node<K, V> { link => RBTreeLink });

impl<'a, K: Ord + 'a, V> KeyAdapter<'a> for NodeAdapter<K, V> {
    type Key = Relative<&'a K>;

    fn get_key(&self, node: &'a Node<K, V>) -> Self::Key {
        node.interval.left_relative()
    }
}

pub struct Entry<'a, K, V> {
    phantom: PhantomData<(&'a K, &'a V)>,
}
impl<'a, K, V> Entry<'a, K, V> {
    fn interval(&self) -> &Interval<K> {
        todo!()
    }

    fn get(&self) -> &'a V {
        todo!()
    }

    fn next(&self) -> Option<Self> {
        todo!()
    }

    fn prev(&self) -> Option<Self> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = (&Interval<K>, &V)> {
        todo!();
        core::iter::empty()
    }

    fn rev_iter(&self) -> impl Iterator<Item = (&Interval<K>, &V)> {
        todo!();
        core::iter::empty()
    }
}

pub struct EntryMut<'a, K, V> {
    phantom: PhantomData<(&'a K, &'a mut V)>,
}
impl<'a, K, V> EntryMut<'a, K, V> {
    fn as_entry(&self) -> Entry<'_, K, V> {
        todo!()
    }

    fn interval(&self) -> &Interval<K> {
        todo!()
    }

    // TODO: return interval
    fn set_interval(self, interval: Interval<K>) -> (Option<Self>, Vec<(Interval<K>, V)>) {
        todo!()
    }

    // TODO: return interval
    fn set_interval_left(self, left: Endpoint<K>) -> (Option<Self>, Vec<(Interval<K>, V)>) {
        todo!()
    }

    // TODO: return interval
    fn set_interval_right(self, right: Endpoint<K>) -> (Option<Self>, Vec<(Interval<K>, V)>) {
        todo!()
    }

    fn get(&self) -> &'a V {
        todo!()
    }

    fn get_mut(&mut self) -> &'a mut V {
        todo!()
    }

    fn set(&mut self, value: V) -> V {
        todo!()
    }

    // TODO: return interval
    fn remove(self) -> (Option<Self>, Interval<K>, V) {
        todo!()
    }

    fn next(&self) -> Option<Self> {
        todo!()
    }

    fn prev(&self) -> Option<Self> {
        todo!()
    }

    fn iter(&self) -> impl Iterator<Item = (&Interval<K>, &V)> {
        todo!();
        core::iter::empty()
    }

    fn rev_iter(&self) -> impl Iterator<Item = (&Interval<K>, &V)> {
        todo!();
        core::iter::empty()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Interval<K>, &mut V)> {
        todo!();
        core::iter::empty()
    }

    fn rev_iter_mut(&mut self) -> impl Iterator<Item = (&Interval<K>, &mut V)> {
        todo!();
        core::iter::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod relative {
        use core::iter::once;

        use super::*;

        #[test]
        fn cmp() {
            // `Relative` values sorted as expected.
            let values: Vec<_> = once(Relative::LeftInfinity)
                .chain(
                    [7, 10]
                        .into_iter()
                        .flat_map(|n| [Relative::Before(n), Relative::At(n), Relative::After(n)]),
                )
                .chain(once(Relative::RightInfinity))
                .collect();

            for (n1, v1) in values.iter().enumerate() {
                for (n2, v2) in values.iter().enumerate() {
                    let expected = n1.cmp(&n2);

                    assert_eq!(v1.cmp(v2), expected, "{v1:?} {v2:?}");
                }
            }
        }
    }
}
