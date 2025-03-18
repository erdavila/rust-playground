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
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
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

    macro_rules! assert_state {
        ($mbh:ident, $expected_len:literal) => {{
            use hlist::HList;

            let expected_len = $expected_len;

            assert_eq!($mbh.records.len(), expected_len);

            assert_eq!($mbh.facets.len(), 3);

            $mbh.facets.as_ref().enumerate().for_each({
                // type Idxs = <(Name, Youngest, Oldest) as $crate::FacetsTuple<Person>>::Indexes;
                struct FE<'a, Idxs: $crate::Indexes> {
                    records: &'a Vec<Record<Person, Idxs>>,
                }
                impl<I: hlist::index::Index, F: $crate::Facet<Person>, Idxs: Indexes>
                    hlist::ForEach<(I, &F)> for FE<'_, Idxs>
                where
                    Idxs: hlist::GetByIndex<I, Output = usize>,
                {
                    fn for_each(&mut self, (facet_idx, facet): (I, &F)) {
                        // Check reciprocal indexes
                        for (i, rec) in self.records.iter().enumerate() {
                            let z: &Idxs = &rec.entry.queue_indexes;
                            let queue_idx = z.get_by_index(facet_idx);
                            let entry_idx = self.records[*queue_idx]
                                .elem_indexes
                                .get_by_index(facet_idx);
                            assert_eq!(i, *entry_idx);
                        }

                        // Check heapness
                        for (child_i, child_rec) in self.records.iter().enumerate().skip(1) {
                            let child_entry_idx = child_rec.elem_indexes.get_by_index(facet_idx);
                            let child_elem = &self.records[*child_entry_idx].entry.elem;
                            let child_facet = facet.facet(child_elem);

                            let parent_i = (child_i - 1) / 2;
                            let parent_rec = &self.records[parent_i];
                            let parent_entry_idx = parent_rec.elem_indexes.get_by_index(facet_idx);
                            let parent_elem = &self.records[*parent_entry_idx].entry.elem;
                            let parent_facet = facet.facet(parent_elem);

                            match F::PRIORITY {
                                Priority::Min => assert!(parent_facet <= child_facet),
                                Priority::Max => assert!(parent_facet >= child_facet),
                            }
                        }
                    }
                }
                FE {
                    records: &$mbh.records,
                }
            });

            assert_eq!($mbh.len(), expected_len);
            if expected_len == 0 {
                assert!($mbh.is_empty());
            } else {
                assert!(!$mbh.is_empty());
            }
        }};
    }

    #[test]
    fn with_facets() {
        let mbh = MultiBinaryHeap::with_facets((Name, Youngest, Oldest));

        {
            use hlist::HList;

            let expected_len = 0;

            assert_eq!(mbh.records.len(), expected_len);

            assert_eq!(mbh.facets.len(), 3);
            mbh.facets.as_ref().enumerate().for_each({
                // type Idxs = <(Name, Youngest, Oldest) as crate::FacetsTuple<Person>>::Indexes;
                struct FE<'a, T, Idxs: crate::Indexes> {
                    records: &'a Vec<Record<T, Idxs>>,
                }
                impl<I: hlist::index::Index, F: crate::Facet<T>, T, Idxs: crate::Indexes + hlist::GetByIndex<I, Output = usize>>
                    hlist::ForEach<(I, &F)> for FE<'_, T, Idxs>
                {
                    fn for_each(&mut self, (facet_idx, facet): (I, &F)) {
                        for (i, rec) in self.records.iter().enumerate() {
                            let z: &Idxs = &rec.entry.queue_indexes;
                            let queue_idx = hlist::HCons::get_by_index(z, facet_idx); // ?!
                            let queue_idx = rec.entry.queue_indexes.get_by_index(facet_idx);
                            let entry_idx = self.records[*queue_idx]
                                .elem_indexes
                                .get_by_index(facet_idx);
                            assert_eq!(i, *entry_idx);
                        }
                        for (child_i, child_rec) in self.records.iter().enumerate().skip(1) {
                            let child_entry_idx = child_rec.elem_indexes.get_by_index(facet_idx);
                            let child_elem = &self.records[*child_entry_idx].entry.elem;
                            let child_facet = facet.facet(child_elem);
                            let parent_i = (child_i - 1) / 2;
                            let parent_rec = &self.records[parent_i];
                            let parent_entry_idx = parent_rec.elem_indexes.get_by_index(facet_idx);
                            let parent_elem = &self.records[*parent_entry_idx].entry.elem;
                            let parent_facet = facet.facet(parent_elem);
                            match F::PRIORITY {
                                Priority::Min => assert!(parent_facet <= child_facet),
                                Priority::Max => assert!(parent_facet >= child_facet),
                            }
                        }
                    }
                }
                FE {
                    records: &mbh.records,
                }
            });
            assert_eq!(mbh.len(), expected_len);
            if expected_len == 0 {
                assert!(mbh.is_empty());
            } else {
                assert!(!mbh.is_empty());
            }
        };
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
