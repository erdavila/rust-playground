#![expect(private_bounds)]

mod facets {
    use hlist::{HCons, HList, HNil};

    use crate::Facet;

    use super::Indexes;

    pub trait Facets<T>: HList {
        type Indexes: Indexes;

        fn zero_indexes() -> Self::Indexes;
    }

    impl<T> Facets<T> for HNil {
        type Indexes = HNil;

        fn zero_indexes() -> Self::Indexes {
            HNil
        }
    }

    impl<T, F: Facet<T>, Tail: Facets<T>> Facets<T> for HCons<F, Tail> {
        type Indexes = HCons<usize, Tail::Indexes>;

        fn zero_indexes() -> Self::Indexes {
            HCons::new(0, Tail::zero_indexes())
        }
    }

    #[cfg(test)]
    #[test]
    fn test() {
        use crate::tests::{Name, Oldest, Youngest};
        use hlist::hlist;
        type Fs = HCons<Name, HCons<Youngest, HCons<Oldest, HNil>>>;

        let indexes = Fs::zero_indexes();

        assert_eq!(indexes, hlist!(0, 0, 0));
    }
}
pub(crate) use facets::*;

mod indexes {
    use hlist::{HCons, HList, HNil};

    pub(crate) trait Indexes: HList {}

    impl Indexes for HNil {}

    impl<Tail: Indexes> Indexes for HCons<usize, Tail> {}
}
pub(crate) use indexes::*;

mod heaps {
    use hlist::{HCons, HList, HNil};

    use crate::{
        heap::Heap,
        index_ref::{IndexNumber, IndexRefMaker, MakeIndexRef, Succ},
        Facet,
    };

    use super::{Facets, Indexes};

    pub trait Heaps<T, Idxs: Indexes, IdxNum: IndexNumber>: Facets<T> {
        type Type: HList;
    }

    impl<T, Idxs: Indexes, IdxNum: IndexNumber> Heaps<T, Idxs, IdxNum> for HNil {
        type Type = HNil;
    }

    impl<T, Idxs: Indexes, IdxNum: IndexNumber, F, Tail> Heaps<T, Idxs, IdxNum> for HCons<F, Tail>
    where
        F: Facet<T>,
        Tail: Facets<T> + Heaps<T, Idxs, Succ<IdxNum>>,
        IndexRefMaker<Idxs, IdxNum>: MakeIndexRef,
    {
        type Type = HCons<
            Heap<T, F, <IndexRefMaker<Idxs, IdxNum> as MakeIndexRef>::IndexRef>,
            <Tail as Heaps<T, Idxs, Succ<IdxNum>>>::Type,
        >;
    }

    #[cfg(test)]
    #[test]
    fn test() {
        use crate::{
            index_ref::Zero,
            tests::{Name, Oldest, Person, Youngest},
            FacetsTuple,
        };
        use hlist::hlist_type;
        use std::any::TypeId;

        type FsTuple = (Name, Youngest, Oldest);
        type ExpectedIdxs = hlist_type!(usize, usize, usize);
        type IndexRef<IdxNum> = <IndexRefMaker<ExpectedIdxs, IdxNum> as MakeIndexRef>::IndexRef;

        type ExpectedNameHeap = Heap<Person, Name, IndexRef<Zero>>;
        type ExpectedYoungestHeap = Heap<Person, Youngest, IndexRef<Succ<Zero>>>;
        type ExpectedOldestHeap = Heap<Person, Oldest, IndexRef<Succ<Succ<Zero>>>>;
        type Expected = hlist_type!(ExpectedNameHeap, ExpectedYoungestHeap, ExpectedOldestHeap);

        assert_eq!(
            TypeId::of::<<FsTuple as FacetsTuple<Person>>::Heaps>(),
            TypeId::of::<Expected>(),
        );
    }
}
pub(crate) use heaps::*;
