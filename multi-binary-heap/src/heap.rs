#![expect(private_bounds)]

use std::{cell::RefCell, rc::Rc};

use crate::{heaps_build::Indexes, index_ref::IndexRef, Facet};

pub(crate) struct Entry<T, I: Indexes> {
    elem: T,
    indexes: I,
}

type EntryRef<T, I> = Rc<RefCell<Entry<T, I>>>;

pub struct Heap<T, F: Facet<T>, IR: IndexRef> {
    pub(crate) entries: Vec<EntryRef<T, IR::Indexes>>,
    pub(crate) facet: F,
}
impl<T, F: Facet<T>, IR: IndexRef> Heap<T, F, IR> {
    pub fn new(facet: F) -> Self {
        Self {
            entries: Vec::new(),
            facet,
        }
    }
}
impl<T, F: Facet<T>, IR: IndexRef> Clone for Heap<T, F, IR>
where
    T: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        // Should clone each T, instead of cloning the Rc<RefCell<Entry<T, ...>>>
        todo!()
    }
}
