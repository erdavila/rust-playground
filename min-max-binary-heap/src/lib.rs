mod heap;

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::TryReserveError,
    fmt::{Debug, Display},
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
};

use heap::{Entry, EntryRef, Heap, HeapOrder, Max, Min};

#[derive(Clone)]
pub struct MinMaxBinaryHeap<T> {
    min_heap: Heap<T, Min>,
    max_heap: Heap<T, Max>,
}

impl<T> MinMaxBinaryHeap<T>
where
    T: Ord,
{
    pub fn append(&mut self, other: &mut Self) {
        todo!()
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.max_heap.capacity()
    }

    pub fn clear(&mut self) {
        self.min_heap.clear();
        self.max_heap.clear();
    }

    pub fn drain(&mut self) -> Drain<T> {
        self.min_heap.drain();
        Drain(self.max_heap.drain())
    }

    pub fn drain_sorted_from_min(&mut self) -> DrainSorted<T> {
        todo!()
    }

    pub fn drain_sorted_from_max(&mut self) -> DrainSorted<T> {
        todo!()
    }

    #[must_use]
    pub fn into_iter_sorted_from_min(self) -> IntoIterSorted<T> {
        todo!()
    }

    #[must_use]
    pub fn into_iter_sorted_from_max(self) -> IntoIterSorted<T> {
        todo!()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.max_heap.is_empty()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        Iter(self.max_heap.iter())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.max_heap.len()
    }

    #[must_use]
    pub const fn new() -> Self {
        MinMaxBinaryHeap {
            min_heap: Heap::new(),
            max_heap: Heap::new(),
        }
    }

    #[must_use]
    pub fn peek_min(&self) -> Option<Peek<T>> {
        self.peek::<Min>()
    }

    #[must_use]
    pub fn peek_max(&self) -> Option<Peek<T>> {
        self.peek::<Max>()
    }

    fn peek<'a, O>(&'a self) -> Option<Peek<'a, T>>
    where
        O: HeapOrder + 'a,
    {
        let (heap, _) = O::select_heaps(&self.min_heap, &self.max_heap);
        heap.peek()
            .map(|entry| Peek(Ref::map(entry, |e| &e.element)))
    }

    pub fn peek_min_mut(&mut self) -> Option<PeekMut<T>> {
        self.peek_mut::<Min>()
    }

    pub fn peek_max_mut(&mut self) -> Option<PeekMut<T>> {
        self.peek_mut::<Max>()
    }

    fn peek_mut<'a, O>(&'a mut self) -> Option<PeekMut<'a, T>>
    where
        O: HeapOrder + 'a,
    {
        let min_max_binary_heap = NonNull::from(&*self);
        let (heap, _) = O::select_heaps_mut(&mut self.min_heap, &mut self.max_heap);

        heap.peek_mut().map(|entry| PeekMut {
            entry: Some(entry),
            min_max_binary_heap,
        })
    }

    pub fn pop_min(&mut self) -> Option<T> {
        self.pop::<Min>()
    }

    pub fn pop_max(&mut self) -> Option<T> {
        self.pop::<Max>()
    }

    fn pop<O>(&mut self) -> Option<T>
    where
        O: HeapOrder,
    {
        let (heap, other_heap) = O::select_heaps_mut(&mut self.min_heap, &mut self.max_heap);
        if let Some(entry) = heap.pop() {
            let other_heap_index = O::Other::get_heap_index(&entry.borrow());
            other_heap.remove(other_heap_index);

            let value = Entry::ref_into_value(entry);
            Some(value)
        } else {
            None
        }
    }

    pub fn push(&mut self, element: T) {
        let index = self.len();

        let entry = Rc::new(RefCell::new(Entry {
            element,
            min_heap_index: index,
            max_heap_index: index,
        }));

        self.min_heap.push(Rc::clone(&entry));
        self.max_heap.push(entry);
    }

    pub fn reserve(&mut self, additional: usize) {
        self.min_heap.reserve(additional);
        self.max_heap.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.min_heap.reserve_exact(additional);
        self.max_heap.reserve_exact(additional);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        todo!()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.min_heap.shrink_to(min_capacity);
        self.max_heap.shrink_to(min_capacity);
    }

    pub fn shrink_to_fit(&mut self) {
        self.min_heap.shrink_to_fit();
        self.max_heap.shrink_to_fit();
    }

    /// # Errors
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.min_heap.try_reserve(additional)?;
        self.max_heap.try_reserve(additional)
    }

    /// # Errors
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.min_heap.try_reserve_exact(additional)?;
        self.max_heap.try_reserve_exact(additional)
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        MinMaxBinaryHeap {
            min_heap: Heap::with_capacity(capacity),
            max_heap: Heap::with_capacity(capacity),
        }
    }
}
impl<T> Debug for MinMaxBinaryHeap<T>
where
    T: Debug + Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
impl<T> Default for MinMaxBinaryHeap<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<'a, T> Extend<&'a T> for MinMaxBinaryHeap<T>
where
    T: 'a + Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        todo!()
    }
}
impl<T> Extend<T> for MinMaxBinaryHeap<T>
where
    T: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        todo!()
    }
}
impl<T> FromIterator<T> for MinMaxBinaryHeap<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();

        let (min_size, max_size) = iter.size_hint();
        let capacity = max_size.unwrap_or(min_size);

        let mut heap = MinMaxBinaryHeap::with_capacity(capacity);

        for value in iter {
            heap.push(value);
        }

        heap
    }
}
impl<'a, T> IntoIterator for &'a MinMaxBinaryHeap<T>
where
    T: Ord,
{
    type Item = Ref<'a, T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.max_heap.iter())
    }
}
impl<T> IntoIterator for MinMaxBinaryHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.max_heap.into_iter())
    }
}

pub struct Drain<'a, T>(std::vec::Drain<'a, EntryRef<T>>);
impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Entry::ref_into_value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<T> DoubleEndedIterator for Drain<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(Entry::ref_into_value)
    }
}
impl<T> ExactSizeIterator for Drain<'_, T> {}
impl<T> FusedIterator for Drain<'_, T> {}

pub struct DrainSorted<'a, T> {
    phantom: PhantomData<&'a T>,
}

pub struct IntoIterSorted<T> {
    phantom: PhantomData<T>,
}

pub struct Iter<'a, T>(std::slice::Iter<'a, EntryRef<T>>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|entry| Ref::map(entry.borrow(), |e| &e.element))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|entry| Ref::map(entry.borrow(), |e| &e.element))
    }
}
impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> FusedIterator for Iter<'_, T> {}
impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter(self.0.clone())
    }
}
impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Iter(std::slice::Iter::default())
    }
}

pub struct IntoIter<T>(std::vec::IntoIter<EntryRef<T>>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Entry::ref_into_value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(Entry::ref_into_value)
    }
}
impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}
impl<T> Clone for IntoIter<T> {
    fn clone(&self) -> Self {
        IntoIter(self.0.clone())
    }
}
impl<T> Default for IntoIter<T> {
    fn default() -> Self {
        IntoIter(std::vec::IntoIter::default())
    }
}

pub struct Peek<'a, T>(Ref<'a, T>);
impl<T> Deref for Peek<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Display for Peek<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct PeekMut<'a, T>
where
    T: Ord,
{
    entry: Option<RefMut<'a, Entry<T>>>,
    min_max_binary_heap: NonNull<MinMaxBinaryHeap<T>>,
}
impl<T> PeekMut<'_, T>
where
    T: Ord,
{
    #[must_use]
    pub fn pop(mut this: Self) -> T {
        let Some(entry) = this.entry.take() else {
            unreachable!()
        };

        let min_max_binary_heap = unsafe { this.min_max_binary_heap.as_mut() };
        let entry = {
            min_max_binary_heap.min_heap.remove(entry.min_heap_index);
            min_max_binary_heap.max_heap.remove(entry.max_heap_index)
        };

        Entry::ref_into_value(entry)
    }
}
impl<T> Deref for PeekMut<'_, T>
where
    T: Ord,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.entry.as_ref().unwrap().element
    }
}
impl<T> DerefMut for PeekMut<'_, T>
where
    T: Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry.as_mut().unwrap().element
    }
}
impl<T> Drop for PeekMut<'_, T>
where
    T: Ord,
{
    fn drop(&mut self) {
        if let Some(entry) = self.entry.take() {
            let min_heap_index = entry.min_heap_index;
            let max_heap_index = entry.max_heap_index;
            drop(entry); // Release the RefMut

            // At this point, the referenced value may have changed, so we need to re-heapify the heaps.

            let min_max_binary_heap = unsafe { self.min_max_binary_heap.as_mut() };
            min_max_binary_heap
                .min_heap
                .heap_up_and_down(min_heap_index);
            min_max_binary_heap
                .max_heap
                .heap_up_and_down(max_heap_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! assert_state {
        ($heap:ident) => {{
            assert_eq!(
                $heap.min_heap.len(),
                $heap.max_heap.len(),
                "heap sizes do not match"
            );

            for i in 0..$heap.min_heap.len() {
                let min_heap_entry = $heap.min_heap[i].borrow();
                let max_heap_entry = $heap.max_heap[i].borrow();

                assert_eq!(min_heap_entry.min_heap_index, i, "wrong min_heap_index");
                assert_eq!(max_heap_entry.max_heap_index, i, "wrong max_heap_index");

                if i > 0 {
                    let parent = (i - 1) / 2;
                    assert!(
                        $heap.min_heap[parent].borrow().element <= min_heap_entry.element,
                        "min_heap invariant violated at {i}"
                    );
                    assert!(
                        $heap.max_heap[parent].borrow().element >= max_heap_entry.element,
                        "max_heap invariant violated at {i}"
                    );
                }

                let entry_rc = &$heap.min_heap[i];
                assert!(
                    Rc::ptr_eq(entry_rc, &$heap.max_heap[entry_rc.borrow().max_heap_index]),
                    "min_heap and max_heap entries do not point to the same object at {i}"
                );
            }
        }};
    }

    #[test]
    fn push() {
        let mut heap = MinMaxBinaryHeap::new();

        heap.push(1);
        assert_state!(heap);

        heap.push(2);
        assert_state!(heap);

        heap.push(3);
        assert_state!(heap);

        heap.push(4);
        assert_state!(heap);

        heap.push(5);
        assert_state!(heap);
    }

    #[test]
    fn pop() {
        let mut heap = MinMaxBinaryHeap::new();
        heap.push(2);
        heap.push(1);
        heap.push(8);
        heap.push(4);
        // Current elements: [1, 2, 4, 8]

        assert_eq!(heap.pop_min(), Some(1));
        assert_state!(heap);
        // Current elements: [2, 4, 8]

        heap.push(3);
        // Current elements: [2, 3, 4, 8]

        assert_eq!(heap.pop_max(), Some(8));
        assert_state!(heap);
        // Current elements: [2, 3, 4]

        heap.push(7);
        // Current elements: [2, 3, 4, 7]

        assert_eq!(heap.pop_min(), Some(2));
        assert_state!(heap);
        // Current elements: [3, 4, 7]

        heap.push(5);
        // Current elements: [3, 4, 5, 7]

        assert_eq!(heap.pop_max(), Some(7));
        assert_state!(heap);
        // Current elements: [3, 4, 5]

        heap.push(6);
        // Current elements: [3, 4, 5, 6]

        assert_eq!(heap.pop_min(), Some(3));
        assert_state!(heap);
        // Current elements: [4, 5, 6]

        assert_eq!(heap.pop_max(), Some(6));
        assert_state!(heap);
        // Current elements: [4, 5]

        assert_eq!(heap.pop_min(), Some(4));
        assert_state!(heap);
        // Current elements: [5]

        assert_eq!(heap.pop_max(), Some(5));
        assert_state!(heap);
        // Current elements: []

        assert_eq!(heap.pop_min(), None);
        assert_state!(heap);

        assert_eq!(heap.pop_max(), None);
        assert_state!(heap);
    }

    #[test]
    fn peek() {
        let mut heap = MinMaxBinaryHeap::new();
        // Current elements: []

        assert!(heap.peek_min().is_none());
        assert!(heap.peek_max().is_none());

        heap.push(2);
        // Current elements: [2]

        assert_eq!(heap.peek_min().map(|n| *n), Some(2));
        assert_eq!(heap.peek_max().map(|n| *n), Some(2));

        heap.push(4);
        // Current elements: [2, 4]

        assert_eq!(heap.peek_min().map(|n| *n), Some(2));
        assert_eq!(heap.peek_max().map(|n| *n), Some(4));

        heap.push(3);
        // Current elements: [2, 3, 4]

        assert_eq!(heap.peek_min().map(|n| *n), Some(2));
        assert_eq!(heap.peek_max().map(|n| *n), Some(4));

        heap.push(1);
        // Current elements: [1, 2, 3, 4]

        assert_eq!(heap.peek_min().map(|n| *n), Some(1));
        assert_eq!(heap.peek_max().map(|n| *n), Some(4));

        heap.push(5);
        // Current elements: [1, 2, 3, 4, 5]

        assert_eq!(heap.peek_min().map(|n| *n), Some(1));
        assert_eq!(heap.peek_max().map(|n| *n), Some(5));
    }

    #[test]
    fn peek_mut() {
        let mut heap = MinMaxBinaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        // Current elements: [1, 2, 3]

        if let Some(mut peek_min) = heap.peek_min_mut() {
            assert_eq!(*peek_min, 1);
            *peek_min = 4;
        } else {
            panic!("peek_min_mut returned None");
        }
        // Current elements: [2, 3, 4]

        assert_state!(heap);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.peek_min().map(|n| *n), Some(2));
        assert_eq!(heap.peek_max().map(|n| *n), Some(4));

        if let Some(mut peek_max) = heap.peek_max_mut() {
            assert_eq!(*peek_max, 4);
            *peek_max = 1;
        } else {
            panic!("peek_max_mut returned None");
        }
        // Current elements: [1, 2, 3]

        assert_state!(heap);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.peek_min().map(|n| *n), Some(1));
        assert_eq!(heap.peek_max().map(|n| *n), Some(3));
    }

    #[test]
    fn peek_mut_pop() {
        let mut heap = MinMaxBinaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        // Current elements: [1, 2, 3]

        if let Some(peek_min) = heap.peek_min_mut() {
            assert_eq!(PeekMut::pop(peek_min), 1);
        } else {
            panic!("peek_min_mut returned None");
        }
        // Current elements: [2, 3]

        assert_state!(heap);
        assert_eq!(heap.len(), 2);
        assert_eq!(heap.peek_min().map(|n| *n), Some(2));
        assert_eq!(heap.peek_max().map(|n| *n), Some(3));

        if let Some(peek_max) = heap.peek_max_mut() {
            assert_eq!(PeekMut::pop(peek_max), 3);
        } else {
            panic!("peek_max_mut returned None");
        }
        // Current elements: [2]

        assert_state!(heap);
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.peek_min().map(|n| *n), Some(2));
        assert_eq!(heap.peek_max().map(|n| *n), Some(2));
    }

    #[test]
    fn iter() {
        let expected: HashSet<_> = (1..=5).collect();

        let heap = {
            let mut h = MinMaxBinaryHeap::new();
            for n in expected.iter() {
                h.push(*n);
            }
            h
        };

        let mut iter = heap.iter();

        let first: Option<Ref<i32>> = iter.next();
        assert_eq!(first.as_ref().map(|n| **n), expected.iter().max().copied());

        let found: HashSet<_> = first.into_iter().chain(iter).map(|n| *n).collect();
        assert_eq!(found, expected);
    }

    #[test]
    fn into_iter() {
        let expected: HashSet<_> = (1..=5).collect();

        let heap = {
            let mut h = MinMaxBinaryHeap::new();
            for n in expected.iter() {
                h.push(*n);
            }
            h
        };

        let mut iter = heap.into_iter();

        let first: Option<i32> = iter.next();
        assert_eq!(first, expected.iter().max().copied());

        let found: HashSet<_> = first.into_iter().chain(iter).collect();
        assert_eq!(found, expected);
    }

    #[test]
    fn into_iter_ref() {
        let expected: HashSet<_> = (1..=5).collect();

        let heap = {
            let mut h = MinMaxBinaryHeap::new();
            for n in expected.iter() {
                h.push(*n);
            }
            h
        };

        let mut iter = (&heap).into_iter();

        let first: Option<Ref<i32>> = iter.next();
        assert_eq!(first.as_ref().map(|n| **n), expected.iter().max().copied());

        let found: HashSet<_> = first.into_iter().chain(iter).map(|n| *n).collect();
        assert_eq!(found, expected);
    }

    #[test]
    fn from_iter() {
        let values = 1..=5;

        let heap = MinMaxBinaryHeap::from_iter(values.clone());

        assert_state!(heap);
        assert_eq!(heap.len(), values.clone().count());
        assert_eq!(
            heap.into_iter().collect::<HashSet<_>>(),
            values.collect::<HashSet<_>>()
        );
    }

    #[test]
    fn from_iter_via_collect() {
        let values = 1..=5;

        let heap: MinMaxBinaryHeap<_> = values.clone().collect();

        assert_state!(heap);
        assert_eq!(heap.len(), values.clone().count());
        assert_eq!(
            heap.into_iter().collect::<HashSet<_>>(),
            values.collect::<HashSet<_>>()
        );
    }

    #[test]
    fn drain() {
        let values = 1..=5;
        let mut heap = MinMaxBinaryHeap::from_iter(values.clone());

        let drained: HashSet<_> = heap.drain().collect();

        assert_state!(heap);
        assert!(heap.is_empty());
        assert_eq!(drained, values.collect::<HashSet<_>>());
    }

    #[test]
    fn drain_not_fully_iterated() {
        let values = 1..=5;
        let mut heap = MinMaxBinaryHeap::from_iter(values);

        let mut drain = heap.drain();
        assert_eq!(drain.next(), Some(5));
        assert!(drain.next().is_some());
        drop(drain);

        assert_state!(heap);
        assert!(heap.is_empty());
    }
}
