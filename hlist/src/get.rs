use crate::{HCons, HList};

/// Implements getting the element at index `N` from an [`HList`].
///
/// Used to implement [`HList::get`] and [`HList::get_mut`].
pub trait Get<const N: usize>: HList {
    /// The type of the element at index `N`.
    type Output;

    /// Gets a reference to the element at index `N`.
    fn get(this: &Self) -> &Self::Output;

    /// Gets a mutable reference to the element at index `N`.
    fn get_mut(this: &mut Self) -> &mut Self::Output;
}

impl<H, T> Get<0> for HCons<H, T>
where
    T: HList,
{
    type Output = H;

    fn get(this: &Self) -> &Self::Output {
        &this.head
    }

    fn get_mut(this: &mut Self) -> &mut Self::Output {
        &mut this.head
    }
}

macro_rules! impl_get_with_positive_index {
    ($index:literal) => {
        impl<H, T> Get<$index> for HCons<H, T>
        where
            T: HList + Get<{ $index - 1 }>,
        {
            type Output = <T as Get<{ $index - 1 }>>::Output;

            fn get(this: &Self) -> &Self::Output {
                Get::<{ $index - 1 }>::get(&this.tail)
            }

            fn get_mut(this: &mut Self) -> &mut Self::Output {
                Get::<{ $index - 1 }>::get_mut(&mut this.tail)
            }
        }
    };
}

impl_get_with_positive_index!(1);
impl_get_with_positive_index!(2);
impl_get_with_positive_index!(3);
impl_get_with_positive_index!(4);
impl_get_with_positive_index!(5);
impl_get_with_positive_index!(6);
impl_get_with_positive_index!(7);
impl_get_with_positive_index!(8);
impl_get_with_positive_index!(9);
impl_get_with_positive_index!(10);
impl_get_with_positive_index!(11);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hlist;

    #[test]
    fn get() {
        let hlist = hlist!(123);
        assert_eq!(hlist.get::<0>(), &123);

        let hlist = hlist!(123, "abc");
        assert_eq!(hlist.get::<0>(), &123);
        assert_eq!(hlist.get::<1>(), &"abc");

        let hlist = hlist!(123, "abc", true);
        assert_eq!(hlist.get::<0>(), &123);
        assert_eq!(hlist.get::<1>(), &"abc");
        assert_eq!(hlist.get::<2>(), &true);

        let hlist = hlist!(123, "abc", true, ['a', 'b']);
        assert_eq!(hlist.get::<0>(), &123);
        assert_eq!(hlist.get::<1>(), &"abc");
        assert_eq!(hlist.get::<2>(), &true);
        assert_eq!(hlist.get::<3>(), &['a', 'b']);
    }

    #[test]
    fn get_mut() {
        let mut hlist = hlist!(123, "abc", true);

        *hlist.get_mut::<0>() = 456;
        *hlist.get_mut::<1>() = "def";
        *hlist.get_mut::<2>() = false;

        assert_eq!(hlist, hlist!(456, "def", false));
    }
}
