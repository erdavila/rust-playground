use hlist::HCons;

use crate::heaps_build::Indexes;

pub(crate) trait IndexRef {
    type Indexes: Indexes;

    fn get(indexes: &Self::Indexes) -> &usize;

    fn get_mut(indexes: &mut Self::Indexes) -> &mut usize;
}

pub struct Head<I: Indexes>(std::marker::PhantomData<I>);
impl<Tail: Indexes> IndexRef for Head<HCons<usize, Tail>> {
    type Indexes = HCons<usize, Tail>;

    fn get(indexes: &HCons<usize, Tail>) -> &usize {
        &indexes.head
    }

    fn get_mut(indexes: &mut Self::Indexes) -> &mut usize {
        &mut indexes.head
    }
}

pub struct Tail<IR: IndexRef>(std::marker::PhantomData<IR>);
impl<IR: IndexRef> IndexRef for Tail<IR> {
    type Indexes = HCons<usize, IR::Indexes>;

    fn get(indexes: &Self::Indexes) -> &usize {
        IR::get(&indexes.tail)
    }

    fn get_mut(indexes: &mut Self::Indexes) -> &mut usize {
        IR::get_mut(&mut indexes.tail)
    }
}

mod index_number {
    pub(crate) trait IndexNumber {}

    pub enum Zero {}
    impl IndexNumber for Zero {}

    pub struct Succ<N: IndexNumber>(std::marker::PhantomData<N>);
    impl<N: IndexNumber> IndexNumber for Succ<N> {}
}
pub(crate) use index_number::*;

mod make_index_ref {
    use hlist::HCons;

    use crate::heaps_build::Indexes;

    use super::{Head, IndexNumber, IndexRef, Succ, Tail, Zero};

    pub trait MakeIndexRef {
        type IndexRef: IndexRef;
    }

    pub struct IndexRefMaker<Idxs: Indexes, IdxNum: IndexNumber> {
        phantom: std::marker::PhantomData<(Idxs, IdxNum)>,
    }

    impl<Tail: Indexes> MakeIndexRef for IndexRefMaker<HCons<usize, Tail>, Zero> {
        type IndexRef = Head<HCons<usize, Tail>>;
    }

    impl<Tl: Indexes, IdxNum: IndexNumber> MakeIndexRef
        for IndexRefMaker<HCons<usize, Tl>, Succ<IdxNum>>
    where
        IndexRefMaker<Tl, IdxNum>: MakeIndexRef,
    {
        type IndexRef = Tail<<IndexRefMaker<Tl, IdxNum> as MakeIndexRef>::IndexRef>;
    }

    #[cfg(test)]
    #[test]
    fn test() {
        use std::any::TypeId;

        use crate::index_ref::Zero;
        use hlist::{hlist, hlist_type, HNil};
        type Idxs = hlist_type!(usize, usize, usize);
        type IndexRef<IdxNum> = <IndexRefMaker<Idxs, IdxNum> as MakeIndexRef>::IndexRef;
        let indexes: Idxs = hlist!(0usize, 1usize, 2usize);

        type Idx0 = IndexRef<Zero>;
        assert_eq!(
            TypeId::of::<Idx0>(),
            TypeId::of::<Head<hlist_type!(usize, usize, usize)>>(),
        );
        assert_eq!(Idx0::get(&indexes) as *const _, &raw const indexes.head);

        type Idx1 = IndexRef<Succ<Zero>>;
        assert_eq!(
            TypeId::of::<Idx1>(),
            TypeId::of::<Tail<Head<hlist_type!(usize, usize)>>>(),
        );
        assert_eq!(
            Idx1::get(&indexes) as *const _,
            &raw const indexes.tail.head
        );

        type Idx2 = IndexRef<Succ<Succ<Zero>>>;
        assert_eq!(
            TypeId::of::<Idx2>(),
            TypeId::of::<Tail<Tail<Head<hlist_type!(usize)>>>>(),
        );
        assert_eq!(
            Idx2::get(&indexes) as *const _,
            &raw const indexes.tail.tail.head
        );
    }
}
pub(crate) use make_index_ref::*;
