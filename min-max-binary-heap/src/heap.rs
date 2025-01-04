use std::{cell::RefCell, marker::PhantomData, rc::Rc};

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

    fn heap_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;

            let mut entry = self.entries[index].borrow_mut();
            let mut parent_entry = self.entries[parent_index].borrow_mut();

            if O::validate_order(&parent_entry.element, &entry.element) {
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

pub(crate) trait HeapOrder {
    fn validate_order<T>(parent_value: &T, child_value: &T) -> bool
    where
        T: Ord;

    fn set_heap_index<T>(entry: &mut Entry<T>, index: usize);
}

pub(crate) enum Min {}
impl HeapOrder for Min {
    fn validate_order<T>(parent_value: &T, child_value: &T) -> bool
    where
        T: Ord,
    {
        parent_value <= child_value
    }

    fn set_heap_index<T>(entry: &mut Entry<T>, index: usize) {
        entry.min_heap_index = index;
    }
}

pub(crate) enum Max {}
impl HeapOrder for Max {
    fn validate_order<T>(parent_value: &T, child_value: &T) -> bool
    where
        T: Ord,
    {
        parent_value >= child_value
    }

    fn set_heap_index<T>(entry: &mut Entry<T>, index: usize) {
        entry.max_heap_index = index;
    }
}
