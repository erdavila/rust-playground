use crate::{
    index::{Index, Succ, Zero},
    HCons, HList,
};

/// Trait for getting a value from its [`Index`].
pub trait GetByIndex<I: Index>: HList {
    /// Type type of the returned value.
    type Output;

    /// Gets a reference to a value given its [`Index`].
    fn get_by_index(this: &Self) -> &Self::Output;

    /// Gets a mutable reference to a value given its [`Index`].
    fn get_by_index_mut(this: &mut Self) -> &mut Self::Output;
}

impl<H, T: HList> GetByIndex<Zero> for HCons<H, T> {
    type Output = H;

    fn get_by_index(this: &Self) -> &Self::Output {
        &this.head
    }

    fn get_by_index_mut(this: &mut Self) -> &mut Self::Output {
        &mut this.head
    }
}

impl<H, T, I: Index> GetByIndex<Succ<I>> for HCons<H, T>
where
    T: GetByIndex<I>,
{
    type Output = <T as GetByIndex<I>>::Output;

    fn get_by_index(this: &Self) -> &Self::Output {
        GetByIndex::get_by_index(&this.tail)
    }

    fn get_by_index_mut(this: &mut Self) -> &mut Self::Output {
        GetByIndex::get_by_index_mut(&mut this.tail)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist;

    use super::*;

    #[test]
    fn get_by_index() {
        let hlist = hlist!(123i32, "abc", true);

        assert_eq!(hlist.get_by_index(Zero), &123);
        assert_eq!(hlist.get_by_index(Succ(Zero)), &"abc");
        assert_eq!(hlist.get_by_index(Succ(Succ(Zero))), &true);
    }

    #[test]
    fn get_by_index_mut() {
        let mut hlist = hlist!(123i32, "abc", true);

        *hlist.get_by_index_mut(Zero) = 456;
        *hlist.get_by_index_mut(Succ(Zero)) = "def";
        *hlist.get_by_index_mut(Succ(Succ(Zero))) = false;

        assert_eq!(hlist, hlist!(456, "def", false));
    }
}
