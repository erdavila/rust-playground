use std::marker::PhantomData;

use crate::{HCons, HList};

pub trait GetByType<A, Where: self::Where> {
    fn get_by_type(this: &Self) -> &A;

    fn get_by_type_mut(this: &mut Self) -> &mut A;
}

impl<A, T> GetByType<A, Here> for HCons<A, T>
where
    T: HList,
{
    fn get_by_type(this: &Self) -> &A {
        &this.head
    }

    fn get_by_type_mut(this: &mut Self) -> &mut A {
        &mut this.head
    }
}

impl<A, H, T, W> GetByType<A, There<W>> for HCons<H, T>
where
    T: HList + GetByType<A, W>,
    W: Where,
{
    fn get_by_type(this: &Self) -> &A {
        GetByType::get_by_type(&this.tail)
    }

    fn get_by_type_mut(this: &mut Self) -> &mut A {
        GetByType::get_by_type_mut(&mut this.tail)
    }
}

pub trait Where {}

pub enum Here {}
impl Where for Here {}

pub struct There<W: Where>(PhantomData<W>);
impl<W: Where> Where for There<W> {}

#[cfg(test)]
mod tests {
    use crate::hlist;

    #[test]
    fn get_by_type() {
        let hlist = hlist!(123i32, "abc", true);

        let value: &i32 = hlist.get_by_type();
        assert_eq!(value, &123);

        let value: &&str = hlist.get_by_type();
        assert_eq!(value, &"abc");

        let value: &bool = hlist.get_by_type();
        assert_eq!(value, &true);

        assert_eq!(hlist.get_by_type::<i32, _>(), &123);
        assert_eq!(hlist.get_by_type::<&str, _>(), &"abc");
        assert_eq!(hlist.get_by_type::<bool, _>(), &true);
    }

    #[test]
    fn get_by_type_mut() {
        let mut hlist = hlist!(123i32, "abc", true);

        *hlist.get_by_type_mut() = 456;
        *hlist.get_by_type_mut() = "def";
        *hlist.get_by_type_mut() = false;

        assert_eq!(hlist, hlist!(456, "def", false));
    }
}
