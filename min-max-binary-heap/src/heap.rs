use std::{
    cell::{Ref, RefCell},
    marker::PhantomData,
    rc::Rc,
};

#[cfg(test)]
use std::ops::Index;

pub(crate) struct Entry<T> {
    pub element: T,
    pub min_heap_index: usize,
    pub max_heap_index: usize,
}

type EntryRef<T> = Rc<RefCell<Entry<T>>>;

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

    pub(crate) fn push(&mut self, entry: EntryRef<T>) {
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

    pub(crate) fn peek(&self) -> Option<Ref<Entry<T>>> {
        self.entries.first().map(|entry| entry.borrow())
    }

    pub(crate) fn remove(&mut self, index: usize) -> EntryRef<T> {
        let entry = self.entries.swap_remove(index);

        if index < self.entries.len() {
            O::set_heap_index(&mut self.entries[index].borrow_mut(), index);
            self.heap_down(index);
            self.heap_up(index);
        }

        entry
    }

    fn heap_up(&mut self, mut index: usize) {
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
}
#[cfg(test)]
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
