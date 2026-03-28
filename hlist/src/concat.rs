use std::ops::Add;

use crate::{HCons, HList, HNil};

/// Trait for concatenating two [`HList`]s.
pub trait Concat<HL>: HList {
    /// The resulting [`HList`] after concatenating two [`HList`]s.
    type Output: HList;

    /// Concatenates two [`HList`]s.
    fn concat(this: Self, other: HL) -> Self::Output;
}

impl<HL> Concat<HL> for HNil
where
    HL: HList,
{
    type Output = HL;

    fn concat(_this: Self, other: HL) -> Self::Output {
        other
    }
}

impl<H, T, HL> Concat<HL> for HCons<H, T>
where
    T: Concat<HL> + HList,
    HL: HList,
{
    type Output = HCons<H, <T as Concat<HL>>::Output>;

    fn concat(this: Self, other: HL) -> Self::Output {
        HCons {
            head: this.head,
            tail: Concat::concat(this.tail, other),
        }
    }
}

impl<HL> Add<HL> for HNil
where
    HL: HList,
{
    type Output = <HNil as Concat<HL>>::Output;

    fn add(self, rhs: HL) -> Self::Output {
        Concat::concat(self, rhs)
    }
}
impl<H, T, HL> Add<HL> for HCons<H, T>
where
    T: Concat<HL> + HList,
    HL: HList,
{
    type Output = <HCons<H, T> as Concat<HL>>::Output;

    fn add(self, rhs: HL) -> Self::Output {
        Concat::concat(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hlist;

    #[test]
    fn concat() {
        assert_eq!(hlist!().concat(hlist!()), hlist!());

        assert_eq!(hlist!().concat(hlist!(123)), hlist!(123));
        assert_eq!(hlist!(123).concat(hlist!()), hlist!(123));

        assert_eq!(hlist!().concat(hlist!(123, "abc")), hlist!(123, "abc"));
        assert_eq!(hlist!(123).concat(hlist!("abc")), hlist!(123, "abc"));
        assert_eq!(hlist!(123, "abc").concat(hlist!()), hlist!(123, "abc"));

        assert_eq!(
            hlist!().concat(hlist!(123, "abc", true)),
            hlist!(123, "abc", true)
        );
        assert_eq!(
            hlist!(123).concat(hlist!("abc", true)),
            hlist!(123, "abc", true)
        );
        assert_eq!(
            hlist!(123, "abc").concat(hlist!(true)),
            hlist!(123, "abc", true)
        );
        assert_eq!(
            hlist!(123, "abc", true).concat(hlist!()),
            hlist!(123, "abc", true)
        );
    }

    #[test]
    fn add() {
        assert_eq!(hlist!() + hlist!(), hlist!());

        assert_eq!(hlist!() + hlist!(123), hlist!(123));
        assert_eq!(hlist!(123) + hlist!(), hlist!(123));

        assert_eq!(hlist!() + hlist!(123, "abc"), hlist!(123, "abc"));
        assert_eq!(hlist!(123) + hlist!("abc"), hlist!(123, "abc"));
        assert_eq!(hlist!(123, "abc") + hlist!(), hlist!(123, "abc"));

        assert_eq!(
            hlist!() + hlist!(123, "abc", true),
            hlist!(123, "abc", true)
        );
        assert_eq!(hlist!(123) + hlist!("abc", true), hlist!(123, "abc", true));
        assert_eq!(hlist!(123, "abc") + hlist!(true), hlist!(123, "abc", true));
        assert_eq!(
            hlist!(123, "abc", true) + hlist!(),
            hlist!(123, "abc", true)
        );
    }
}
