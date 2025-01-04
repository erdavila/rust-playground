mod heap;

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::TryReserveError,
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
};

use heap::{Entry, Heap, HeapOrder, Max, Min};

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
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn drain(&mut self) -> Drain<T> {
        todo!()
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
        todo!()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        todo!()
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

            let Some(entry) = Rc::into_inner(entry) else {
                unreachable!()
            };
            let entry = RefCell::into_inner(entry);
            Some(entry.element)
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
        todo!()
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        todo!()
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        todo!()
    }

    pub fn shrink_to_fit(&mut self) {
        todo!()
    }

    /// # Errors
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        todo!()
    }

    /// # Errors
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        todo!()
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }
}
impl<T> Clone for MinMaxBinaryHeap<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<T> Debug for MinMaxBinaryHeap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl<T> Default for MinMaxBinaryHeap<T>
where
    T: Ord,
{
    fn default() -> Self {
        todo!()
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
        todo!()
    }
}
impl<'a, T> IntoIterator for &'a MinMaxBinaryHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
impl<T> IntoIterator for MinMaxBinaryHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct Drain<'a, T> {
    phantom: PhantomData<&'a T>,
}

pub struct DrainSorted<'a, T> {
    phantom: PhantomData<&'a T>,
}

pub struct IntoIterSorted<T> {
    phantom: PhantomData<T>,
}

pub struct Iter<'a, T> {
    phantom: PhantomData<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IntoIter<T> {
    phantom: PhantomData<T>,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
        let entry = self.entry.take().unwrap();
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

#[cfg(test)]
mod tests {
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
}
