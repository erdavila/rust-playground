use crate::{HCons, HList, HNil};

/// Trait for zipping two [`HList`]s.
pub trait Zip<HL>: HList {
    /// The resulting [`HList`] after zipping two [`HList`]s.
    type Output: HList;

    /// Zips two [`HList`]s.
    fn zip(this: Self, other: HL) -> Self::Output;
}

impl Zip<HNil> for HNil {
    type Output = HNil;

    fn zip(_: Self, _: HNil) -> Self::Output {
        HNil
    }
}

impl<H1, T1, H2, T2> Zip<HCons<H2, T2>> for HCons<H1, T1>
where
    T1: Zip<T2>,
    T2: HList,
{
    type Output = HCons<(H1, H2), <T1 as Zip<T2>>::Output>;

    fn zip(this: Self, other: HCons<H2, T2>) -> Self::Output {
        HCons {
            head: (this.head, other.head),
            tail: Zip::zip(this.tail, other.tail),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hlist;

    #[test]
    fn zip() {
        assert_eq!(hlist!().zip(hlist!()), hlist!());

        assert_eq!(hlist!(123).zip(hlist!("abc")), hlist!((123, "abc")));

        assert_eq!(
            hlist!(123, "abc").zip(hlist!(true, ['a', 'b'])),
            hlist!((123, true), ("abc", ['a', 'b']))
        );
    }
}
