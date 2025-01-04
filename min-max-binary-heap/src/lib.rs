use std::{cell::RefCell, collections::TryReserveError, fmt::Debug, marker::PhantomData, rc::Rc};

pub struct MinMaxBinaryHeap<T> {
    min_heap: Vec<Rc<RefCell<Entry<T>>>>,
    max_heap: Vec<Rc<RefCell<Entry<T>>>>,
}

struct Entry<T> {
    element: T,
    min_heap_index: usize,
    max_heap_index: usize,
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
            min_heap: Vec::new(),
            max_heap: Vec::new(),
        }
    }

    #[must_use]
    pub fn peek_min(&self) -> Option<&T> {
        todo!()
    }

    #[must_use]
    pub fn peek_max(&self) -> Option<&T> {
        todo!()
    }

    pub fn peek_min_mut(&mut self) -> Option<PeekMut<T>> {
        todo!()
    }

    pub fn peek_max_mut(&mut self) -> Option<PeekMut<T>> {
        todo!()
    }

    pub fn pop_min(&mut self) -> Option<T> {
        todo!()
    }

    pub fn pop_max(&mut self) -> Option<T> {
        todo!()
    }

    pub fn push(&mut self, element: T) {
        let index = self.len();

        let entry = Rc::new(RefCell::new(Entry {
            element,
            min_heap_index: index,
            max_heap_index: index,
        }));

        self.min_heap.push(Rc::clone(&entry));
        heap_up_min(index, &mut self.min_heap);

        self.max_heap.push(entry);
        heap_up_max(index, &mut self.max_heap);
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

fn heap_up_min<T>(mut index: usize, min_heap: &mut [Rc<RefCell<Entry<T>>>])
where
    T: Ord,
{
    while index > 0 {
        let parent_index = (index - 1) / 2;

        let mut entry = min_heap[index].borrow_mut();
        let mut parent_entry = min_heap[parent_index].borrow_mut();

        if parent_entry.element <= entry.element {
            break;
        }

        parent_entry.min_heap_index = index;
        entry.min_heap_index = parent_index;

        drop(entry);
        drop(parent_entry);
        min_heap.swap(index, parent_index);

        index = parent_index;
    }
}

fn heap_up_max<T>(mut index: usize, max_heap: &mut [Rc<RefCell<Entry<T>>>])
where
    T: Ord,
{
    while index > 0 {
        let parent_index = (index - 1) / 2;

        let mut entry = max_heap[index].borrow_mut();
        let mut parent_entry = max_heap[parent_index].borrow_mut();

        if parent_entry.element >= entry.element {
            break;
        }

        entry.max_heap_index = parent_index;
        parent_entry.max_heap_index = index;

        drop(entry);
        drop(parent_entry);
        max_heap.swap(index, parent_index);

        index = parent_index;
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

pub struct PeekMut<'a, T> {
    phantom: PhantomData<&'a T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_state {
        ($heap:ident) => {{
            assert_eq!($heap.min_heap.len(), $heap.max_heap.len());

            for i in 0..$heap.min_heap.len() {
                let min_heap_entry = $heap.min_heap[i].borrow();
                let max_heap_entry = $heap.max_heap[i].borrow();

                assert_eq!(min_heap_entry.min_heap_index, i);
                assert_eq!(max_heap_entry.max_heap_index, i);

                if i > 0 {
                    let parent = (i - 1) / 2;
                    assert!($heap.min_heap[parent].borrow().element <= min_heap_entry.element);
                    assert!($heap.max_heap[parent].borrow().element >= max_heap_entry.element);
                }

                let entry_rc = &$heap.min_heap[i];
                assert!(Rc::ptr_eq(
                    entry_rc,
                    &$heap.max_heap[entry_rc.borrow().max_heap_index]
                ));
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
}
