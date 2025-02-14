#![expect(private_bounds)]

use heaps_build::{Facets, Heaps};
use hlist::{tuples::Tuple, HList};
use index_ref::Zero;

mod heap;
mod heaps_build;
mod index_ref;

pub trait Facet<T> {
    type Output<'a>: Ord
    where
        T: 'a;

    const PRIORITY: Priority;

    fn facet<'a>(&self, elem: &'a T) -> Self::Output<'a>;
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Priority {
    Min,
    Max,
}

pub trait FacetsTuple<T>: Tuple {
    type Heaps: HList;
}

impl<Tup, T> FacetsTuple<T> for Tup
where
    Tup: Tuple,
    Tup::HList: Heaps<T, <Tup::HList as Facets<T>>::Indexes, Zero>,
{
    type Heaps = <Tup::HList as Heaps<T, <Tup::HList as Facets<T>>::Indexes, Zero>>::Type;
}

pub struct MultiBinaryHeap<T, Fs>
where
    Fs: FacetsTuple<T>,
{
    heaps: Fs::Heaps,
}
impl<T, Fs> MultiBinaryHeap<T, Fs>
where
    Fs: FacetsTuple<T>,
{
    pub fn with_facets(facets: Fs) -> Self {
        todo!()
    }

    pub fn push(&mut self, elem: T) {
        todo!()
    }

    pub fn pop<F>(&mut self) -> Option<T>
    where
        F: Facet<T>,
    {
        todo!()
    }
}
impl<T, Fs> MultiBinaryHeap<T, Fs>
where
    Fs: Default + FacetsTuple<T>,
{
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }
}
impl<T, Fs> Default for MultiBinaryHeap<T, Fs>
where
    Fs: Default + FacetsTuple<T>,
{
    fn default() -> Self {
        MultiBinaryHeap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) struct Person {
        name: String,
        age: u8,
    }

    pub(crate) struct Name;
    impl Facet<Person> for Name {
        type Output<'a> = &'a str;

        const PRIORITY: Priority = Priority::Min;

        fn facet<'a>(&self, elem: &'a Person) -> Self::Output<'a> {
            &elem.name
        }
    }
    impl Default for Name {
        fn default() -> Self {
            Self
        }
    }

    pub(crate) struct Youngest;
    impl Facet<Person> for Youngest {
        type Output<'a> = u8;

        const PRIORITY: Priority = Priority::Min;

        fn facet<'a>(&self, elem: &'a Person) -> Self::Output<'a> {
            elem.age
        }
    }
    impl Default for Youngest {
        fn default() -> Self {
            Self
        }
    }

    pub(crate) struct Oldest;
    impl Facet<Person> for Oldest {
        type Output<'a> = u8;

        const PRIORITY: Priority = Priority::Max;

        fn facet<'a>(&self, elem: &'a Person) -> Self::Output<'a> {
            elem.age
        }
    }
    impl Default for Oldest {
        fn default() -> Self {
            Self
        }
    }

    #[test]
    fn push() {
        todo!();
    }

    #[test]
    fn pop() {
        todo!();
    }

    #[test]
    fn default_facets() {
        todo!();
    }

    #[test]
    fn no_facets() {
        todo!();
    }
}
