use crate::{HCons, HList, HNil};

/// Trait for splitting an [`HList`] at a given index.
pub trait Split<const N: usize> {
    /// The resulting left [`HList`] after splitting.
    type LeftOutput: HList;

    /// The resulting right [`HList`] after splitting.
    type RightOutput: HList;

    /// Splits the [`HList`] at the given index.
    fn split(this: Self) -> (Self::LeftOutput, Self::RightOutput);
}

impl<HL> Split<0> for HL
where
    HL: HList,
{
    type LeftOutput = HNil;

    type RightOutput = HL;

    fn split(this: Self) -> (Self::LeftOutput, Self::RightOutput) {
        (HNil, this)
    }
}

macro_rules! impl_split_with_positive_index {
    ($index:literal) => {
        impl<H, T> Split<$index> for HCons<H, T>
        where
            T: Split<{ $index - 1 }> + HList,
        {
            type LeftOutput = HCons<H, <T as Split<{ $index - 1 }>>::LeftOutput>;

            type RightOutput = <T as Split<{ $index - 1 }>>::RightOutput;

            fn split(this: Self) -> (Self::LeftOutput, Self::RightOutput) {
                let (left_tail, right) = Split::split(this.tail);
                let left = HCons {
                    head: this.head,
                    tail: left_tail,
                };

                (left, right)
            }
        }
    };
}

impl_split_with_positive_index!(1);
impl_split_with_positive_index!(2);
impl_split_with_positive_index!(3);
impl_split_with_positive_index!(4);
impl_split_with_positive_index!(5);
impl_split_with_positive_index!(6);
impl_split_with_positive_index!(7);
impl_split_with_positive_index!(8);
impl_split_with_positive_index!(9);
impl_split_with_positive_index!(10);
impl_split_with_positive_index!(11);
impl_split_with_positive_index!(12);

#[cfg(test)]
mod tests {
    use crate::hlist;

    use super::*;

    #[test]
    fn split() {
        let (left, right) = hlist!().split::<0>();
        assert_eq!(left, hlist!());
        assert_eq!(right, hlist!());

        let (left, right) = hlist!(123).split::<0>();
        assert_eq!(left, hlist!());
        assert_eq!(right, hlist!(123));

        let (left, right) = hlist!(123).split::<1>();
        assert_eq!(left, hlist!(123));
        assert_eq!(right, hlist!());

        let (left, right) = hlist!(123, "abc").split::<0>();
        assert_eq!(left, hlist!());
        assert_eq!(right, hlist!(123, "abc"));

        let (left, right) = hlist!(123, "abc").split::<1>();
        assert_eq!(left, hlist!(123));
        assert_eq!(right, hlist!("abc"));

        let (left, right) = hlist!(123, "abc").split::<2>();
        assert_eq!(left, hlist!(123, "abc"));
        assert_eq!(right, hlist!());
    }
}
