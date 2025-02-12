mod heap;

use heap::Heap;
use hlist::{tuples::Tuple, HCons, HList, HNil};

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

#[allow(private_bounds)]
pub struct MultiBinaryHeap<T, Facets>
where
    Facets: Tuple,
    Facets::HList: FacetsHList<T>,
{
    heaps: <Facets::HList as FacetsHList<T>>::HeapsHList,
}
#[allow(private_bounds)]
impl<T, Facets> MultiBinaryHeap<T, Facets>
where
    Facets: Tuple,
    Facets::HList: FacetsHList<T>,
{
    pub fn with_facets(facets: Facets) -> Self {
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
#[allow(private_bounds)]
impl<T, Facets> MultiBinaryHeap<T, Facets>
where
    Facets: Tuple + Default,
    Facets::HList: FacetsHList<T>,
{
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }
}
impl<T, Facets> Default for MultiBinaryHeap<T, Facets>
where
    Facets: Tuple + Default,
    Facets::HList: FacetsHList<T>,
{
    fn default() -> Self {
        MultiBinaryHeap::new()
    }
}

trait FacetsHList<T>: HList {
    type HeapsHList: HList;
}
impl<T> FacetsHList<T> for HNil {
    type HeapsHList = HNil;
}
impl<T, F: Facet<T>, Tail: FacetsHList<T>> FacetsHList<T> for HCons<F, Tail> {
    type HeapsHList = HCons<Heap<T, F>, Tail::HeapsHList>;
}

#[cfg(test)]
mod tests {
    use super::*;

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
