use hlist::{HCons, HList, HNil};

use crate::Facet;

// The compiler requires the Facets<T> trait to be public.
pub trait Facets<T>: HList {
    type Indexes: Indexes;
}
impl<T> Facets<T> for HNil {
    type Indexes = HNil;
}
impl<T, F: Facet<T>, Tail: Facets<T>> Facets<T> for HCons<F, Tail> {
    type Indexes = HCons<usize, Tail::Indexes>;
}

pub(crate) trait Indexes: HList {}
impl Indexes for HNil {}
impl<Tail: Indexes> Indexes for HCons<usize, Tail> {}
