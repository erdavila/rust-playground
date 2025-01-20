use crate::{HCons, HList, HNil};

/// Implements reversing an [`HList`].
pub trait Rev<Tail>
where
    Self: HList,
    Tail: HList,
{
    /// The output of reversing the list.
    type Output: HList;

    /// Reverses the list.
    fn rev(this: Self, tail: Tail) -> Self::Output;
}
impl<HL> Rev<HL> for HNil
where
    HL: HList,
{
    type Output = HL;

    fn rev(_: Self, tail: HL) -> Self::Output {
        tail
    }
}
impl<H, T, HL> Rev<HL> for HCons<H, T>
where
    T: Rev<HCons<H, HL>>,
    HL: HList,
{
    type Output = <T as Rev<HCons<H, HL>>>::Output;

    fn rev(this: Self, tail: HL) -> Self::Output {
        Rev::rev(
            this.tail,
            HCons {
                head: this.head,
                tail,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist;

    use super::*;

    #[test]
    fn rev() {
        assert_eq!(hlist!().rev(), hlist!());
        assert_eq!(hlist!(123).rev(), hlist!(123));
        assert_eq!(hlist!(123, "abc").rev(), hlist!("abc", 123));
        assert_eq!(hlist!(123, "abc", true).rev(), hlist!(true, "abc", 123));
    }
}
