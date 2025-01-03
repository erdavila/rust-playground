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
        todo!()
    }

    #[must_use]
    pub const fn new() -> Self {
        todo!()
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

    pub fn push(&mut self, value: T) {
        todo!()
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

pub struct PeekMut<'a, T> {
    phantom: PhantomData<&'a T>,
}
