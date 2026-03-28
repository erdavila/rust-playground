use crate::index::{Index, Succ};
use crate::{HCons, HList, HNil};

/// Trait that implements the pairing of values in with their corresponding [`Index`]es.
pub trait Enumerate: HList {
    /// The resulting list, given the [`Index`] for the first element.
    type Output<FirstI: Index>: HList;

    /// Pairs each value with its corresponding [`Index`].
    fn enumerate<FirstI: Index>(this: Self, first_index: FirstI) -> Self::Output<FirstI>;
}

impl Enumerate for HNil {
    type Output<FirstI: Index> = HNil;

    fn enumerate<FirstI: Index>(_: Self, _: FirstI) -> Self::Output<FirstI> {
        HNil
    }
}

impl<H, T> Enumerate for HCons<H, T>
where
    T: Enumerate,
{
    type Output<FirstI: Index> = HCons<(FirstI, H), <T as Enumerate>::Output<Succ<FirstI>>>;

    fn enumerate<FirstI: Index>(this: Self, first_index: FirstI) -> Self::Output<FirstI> {
        HCons::new(
            (first_index, this.head),
            Enumerate::enumerate(this.tail, Succ(first_index)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hlist;
    use crate::index::Zero;

    #[test]
    fn enumerate() {
        let hlist = hlist!(123i32, "abc", true);

        let output = hlist.enumerate();

        assert_eq!(
            output,
            hlist!(
                (Zero, 123i32),
                (Succ(Zero), "abc"),
                (Succ(Succ(Zero)), true),
            )
        );
    }
}
