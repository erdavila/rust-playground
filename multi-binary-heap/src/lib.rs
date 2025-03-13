#![expect(private_bounds)]

mod support;

use hlist::tuples::Tuple;
use support::{Facets, Indexes};

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
    type Facets: Facets<T>;
    type Indexes: Indexes;

    fn into_facets(self) -> Self::Facets;
}
impl<Tup, T> FacetsTuple<T> for Tup
where
    Tup: Tuple,
    Tup::HList: Facets<T>,
{
    type Facets = Tup::HList;
    type Indexes = <Tup::HList as Facets<T>>::Indexes;

    fn into_facets(self) -> Self::Facets {
        self.into_hlist()
    }
}

#[derive(Clone, Debug)]
struct Entry<T, Idxs: Indexes> {
    elem: T,
    queue_indexes: Idxs,
}

#[derive(Clone, Debug)]
struct Record<T, Idxs: Indexes> {
    entry: Entry<T, Idxs>,
    elem_indexes: Idxs,
}

#[derive(Clone, Debug)]
pub struct MultiBinaryHeap<T, Fs>
where
    Fs: FacetsTuple<T>,
{
    facets: Fs::Facets,
    records: Vec<Record<T, Fs::Indexes>>,
}
impl<T, Fs> MultiBinaryHeap<T, Fs>
where
    Fs: FacetsTuple<T>,
{
    pub fn with_facets(facets_tuple: Fs) -> Self {
        MultiBinaryHeap {
            facets: facets_tuple.into_facets(),
            records: Vec::new(),
        }
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

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
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

    struct Person {
        name: String,
        age: u8,
    }

    struct Name;
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

    struct Youngest;
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

    struct Oldest;
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
    fn with_facets() {
        let mbh = MultiBinaryHeap::with_facets((Name, Youngest, Oldest));

        assert_eq!(mbh.facets.len(), 3);
        assert!(mbh.records.is_empty());
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
