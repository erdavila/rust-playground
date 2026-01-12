use std::{
    cell::{Ref, RefCell, RefMut},
    collections::TryReserveError,
    marker::PhantomData,
    ops::Index,
    rc::Rc,
};

pub(crate) struct Entry<T> {
    pub element: T,
    pub min_heap_index: usize,
    pub max_heap_index: usize,
}
impl<T> Entry<T> {
    pub(crate) fn ref_into_value(entry_ref: EntryRef<T>) -> T {
        let Some(entry) = Rc::into_inner(entry_ref) else {
            unreachable!()
        };
        let entry = RefCell::into_inner(entry);
        entry.element
    }
}

pub(crate) type EntryRef<T> = Rc<RefCell<Entry<T>>>;

#[derive(Clone)]
pub(crate) struct Heap<T, O> {
    entries: Vec<EntryRef<T>>,
    phantom: PhantomData<O>,
}
impl<T, O> Heap<T, O>
where
    T: Ord,
    O: HeapOrder,
{
    pub(crate) const fn new() -> Self {
        Self {
            entries: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn append(&mut self, other: &mut Self) {
        for entry in other.drain() {
            self.push(entry);
        }
    }

    pub(crate) fn push(&mut self, entry: EntryRef<T>) {
        O::set_heap_index(&mut entry.borrow_mut(), self.len());
        self.entries.push(entry);
        self.heap_up(self.entries.len() - 1);
    }

    pub(crate) fn pop(&mut self) -> Option<EntryRef<T>> {
        if self.entries.is_empty() {
            None
        } else {
            let entry = self.remove(0);
            Some(entry)
        }
    }

    pub(crate) fn peek(&self) -> Option<Ref<'_, Entry<T>>> {
        self.entries.first().map(|entry| entry.borrow())
    }

    pub(crate) fn peek_mut(&mut self) -> Option<RefMut<'_, Entry<T>>> {
        self.entries.first_mut().map(|entry| entry.borrow_mut())
    }

    pub(crate) fn remove(&mut self, index: usize) -> EntryRef<T> {
        let entry = unsafe { self.swap_remove(index) };
        if index < self.entries.len() {
            self.heap_up_and_down(index);
        }
        entry
    }

    unsafe fn swap_remove(&mut self, index: usize) -> EntryRef<T> {
        let entry = self.entries.swap_remove(index);
        if index < self.entries.len() {
            O::set_heap_index(&mut self.entries[index].borrow_mut(), index);
        }
        entry
    }

    pub(crate) fn retain<F>(&mut self, f: F) -> Retain<'_, T, O, F>
    where
        F: FnMut(&Entry<T>) -> bool,
    {
        Retain {
            heap: self,
            index: 0,
            f,
        }
    }

    pub(crate) fn heap_up_and_down(&mut self, index: usize) {
        self.heap_down(index);
        self.heap_up(index);
    }

    pub(crate) fn heap_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;

            let mut entry = self.entries[index].borrow_mut();
            let mut parent_entry = self.entries[parent_index].borrow_mut();

            if O::comes_before(&parent_entry.element, &entry.element) {
                break;
            }

            O::set_heap_index(&mut parent_entry, index);
            O::set_heap_index(&mut entry, parent_index);

            drop(entry);
            drop(parent_entry);
            self.entries.swap(index, parent_index);

            index = parent_index;
        }
    }

    fn heap_down(&mut self, mut index: usize) {
        while index < self.entries.len() {
            let left_index = 2 * index + 1;
            let right_index = 2 * index + 2;

            if left_index >= self.entries.len() {
                break;
            }

            let mut child_index = left_index;
            if right_index < self.entries.len()
                && O::comes_before(
                    &self.entries[right_index].borrow().element,
                    &self.entries[left_index].borrow().element,
                )
            {
                child_index = right_index;
            }

            let mut entry = self.entries[index].borrow_mut();
            let mut child_entry = self.entries[child_index].borrow_mut();

            if O::comes_before(&entry.element, &child_entry.element) {
                break;
            }

            O::set_heap_index(&mut entry, child_index);
            O::set_heap_index(&mut child_entry, index);

            drop(entry);
            drop(child_entry);
            self.entries.swap(index, child_index);

            index = child_index;
        }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional);
    }

    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.entries.reserve_exact(additional);
    }

    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        self.entries.shrink_to(min_capacity);
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.entries.shrink_to_fit();
    }

    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.entries.try_reserve(additional)
    }

    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.entries.try_reserve_exact(additional)
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            phantom: PhantomData,
        }
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, EntryRef<T>> {
        self.entries.iter()
    }

    pub(crate) fn drain(&mut self) -> std::vec::Drain<'_, EntryRef<T>> {
        self.entries.drain(..)
    }
}
impl<T, O> IntoIterator for Heap<T, O> {
    type Item = EntryRef<T>;
    type IntoIter = std::vec::IntoIter<EntryRef<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
impl<T, O> Index<usize> for Heap<T, O>
where
    O: HeapOrder,
{
    type Output = EntryRef<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

pub(crate) trait HeapOrder: Sized {
    type Other: HeapOrder;

    fn comes_before<T>(a: &T, b: &T) -> bool
    where
        T: Ord;

    fn get_heap_index<T>(entry: &Entry<T>) -> usize;

    fn set_heap_index<T>(entry: &mut Entry<T>, index: usize);

    fn select_heaps<'a, T>(
        min_heap: &'a Heap<T, Min>,
        max_heap: &'a Heap<T, Max>,
    ) -> (&'a Heap<T, Self>, &'a Heap<T, Self::Other>);

    fn select_heaps_mut<'a, T>(
        min_heap: &'a mut Heap<T, Min>,
        max_heap: &'a mut Heap<T, Max>,
    ) -> (&'a mut Heap<T, Self>, &'a mut Heap<T, Self::Other>);
}

#[derive(Clone, Copy)]
pub(crate) enum Min {}
impl HeapOrder for Min {
    type Other = Max;

    fn comes_before<T>(a: &T, b: &T) -> bool
    where
        T: Ord,
    {
        a <= b
    }

    fn get_heap_index<T>(entry: &Entry<T>) -> usize {
        entry.min_heap_index
    }

    fn set_heap_index<T>(entry: &mut Entry<T>, index: usize) {
        entry.min_heap_index = index;
    }

    fn select_heaps<'a, T>(
        min_heap: &'a Heap<T, Min>,
        max_heap: &'a Heap<T, Max>,
    ) -> (&'a Heap<T, Self>, &'a Heap<T, Self::Other>) {
        (min_heap, max_heap)
    }

    fn select_heaps_mut<'a, T>(
        min_heap: &'a mut Heap<T, Min>,
        max_heap: &'a mut Heap<T, Max>,
    ) -> (&'a mut Heap<T, Self>, &'a mut Heap<T, Self::Other>) {
        (min_heap, max_heap)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Max {}
impl HeapOrder for Max {
    type Other = Min;

    fn comes_before<T>(a: &T, b: &T) -> bool
    where
        T: Ord,
    {
        a >= b
    }

    fn get_heap_index<T>(entry: &Entry<T>) -> usize {
        entry.max_heap_index
    }

    fn set_heap_index<T>(entry: &mut Entry<T>, index: usize) {
        entry.max_heap_index = index;
    }

    fn select_heaps<'a, T>(
        min_heap: &'a Heap<T, Min>,
        max_heap: &'a Heap<T, Max>,
    ) -> (&'a Heap<T, Self>, &'a Heap<T, Self::Other>) {
        (max_heap, min_heap)
    }

    fn select_heaps_mut<'a, T>(
        min_heap: &'a mut Heap<T, Min>,
        max_heap: &'a mut Heap<T, Max>,
    ) -> (&'a mut Heap<T, Max>, &'a mut Heap<T, Min>) {
        (max_heap, min_heap)
    }
}

pub(crate) struct Retain<'a, T, O, F>
where
    T: Ord,
    O: HeapOrder,
    F: FnMut(&Entry<T>) -> bool,
{
    heap: &'a mut Heap<T, O>,
    index: usize,
    f: F,
}
impl<T, O, F> Iterator for Retain<'_, T, O, F>
where
    T: Ord,
    O: HeapOrder,
    F: FnMut(&Entry<T>) -> bool,
{
    type Item = EntryRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // valid_heap_in(0..self.index) == true
        // valid_heap_in(0..=self.index) == ?
        // self.index < self.heap.len() == ?

        while self.index < self.heap.len() {
            let entry = self.heap[self.index].borrow();
            let retain = (self.f)(&entry);
            drop(entry);

            // valid_heap_in(0..self.index) == true
            // valid_heap_in(0..=self.index) == ?
            // self.index < self.heap.len() == true

            if retain {
                // The element at self.index is retained

                self.heap.heap_up(self.index);
                // valid_heap_in(0..self.index) == true
                // valid_heap_in(0..=self.index) == true
                // self.index < self.heap.len() == true

                self.index += 1;
                // valid_heap_in(0..self.index) == true
                // valid_heap_in(0..=self.index) == ?
                // self.index <= self.heap.len() == ?
            } else {
                // The element at self.index is replaced with the last element,
                // or removed if it is already the last element.

                let entry = unsafe { self.heap.swap_remove(self.index) };
                // valid_heap_in(0..self.index) == true
                // valid_heap_in(0..=self.index) == ?
                // self.index < self.heap.len() == ?

                return Some(entry);
            }
        }

        // valid_heap_in(0..self.index) == true
        // valid_heap_in(0..=self.index) == false
        // self.index < self.heap.len() == false

        None
    }
}
impl<T, O, F> Drop for Retain<'_, T, O, F>
where
    T: Ord,
    O: HeapOrder,
    F: FnMut(&Entry<T>) -> bool,
{
    fn drop(&mut self) {
        // Ensures evaluation of the remaining elements
        for _ in self {}
    }
}
