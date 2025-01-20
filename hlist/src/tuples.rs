//! Provides conversions of tuples to and from [`HList`]s.
//!
//! Conversions are provided for tuples with up to 12 elements.

use crate::{hcons, HCons, HList, HNil};

/// A trait for tuples.
pub trait Tuple {
    /// The corresponding [`HList`] type.
    type HList: HList;

    /// Turns the tuple into an [`HList`].
    fn into_hlist(self) -> Self::HList;
}

macro_rules! hlist {
    () => {
        HNil
    };
    ( $head:expr $( , $tail:expr )* ) => {
        hcons($head, hlist!( $( $tail ),* ))
    };
}

macro_rules! hlist_type {
    () => {
        HNil
    };
    ( $head:ty $( , $tail:ty )* ) => {
        HCons<$head, hlist_type!( $( $tail ),* )>
    };
}

macro_rules! impl_tuple_trait {
    ($( $type_arg:ident )* ;  $( $index:tt )*) => {
        impl< $( $type_arg ),*  > Tuple for ( $( $type_arg , )* ) {
            type HList = hlist_type!( $( $type_arg ),* );

            fn into_hlist(self) -> Self::HList {
                hlist!( $( self.$index ),* )
            }
        }

        impl< $( $type_arg ),* > From<( $( $type_arg , )* )> for hlist_type!( $( $type_arg ),* ) {
            fn from(tuple: ( $( $type_arg , )* )) -> Self {
                tuple.into_hlist()
            }
        }
    };
}

impl_tuple_trait!(;);
impl_tuple_trait!(A0 ; 0);
impl_tuple_trait!(A0 A1 ; 0 1);
impl_tuple_trait!(A0 A1 A2 ; 0 1 2);
impl_tuple_trait!(A0 A1 A2 A3 ; 0 1 2 3);
impl_tuple_trait!(A0 A1 A2 A3 A4 ; 0 1 2 3 4);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 ; 0 1 2 3 4 5);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 ; 0 1 2 3 4 5 6);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 ; 0 1 2 3 4 5 6 7);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 ; 0 1 2 3 4 5 6 7 8);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 ; 0 1 2 3 4 5 6 7 8 9);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 ; 0 1 2 3 4 5 6 7 8 9 10);
impl_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 ; 0 1 2 3 4 5 6 7 8 9 10 11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_hlist() {
        let hlist = ().into_hlist();
        assert_eq!(hlist, HNil);

        let hlist = (123,).into_hlist();
        assert_eq!(hlist, hcons(123, HNil));

        let hlist = (123, "abc").into_hlist();
        assert_eq!(hlist, hcons(123, hcons("abc", HNil)));

        let hlist = (123, "abc", true).into_hlist();
        assert_eq!(hlist, hcons(123, hcons("abc", hcons(true, HNil))));

        let hlist = (123, "abc", true, ['a', 'b']).into_hlist();
        assert_eq!(
            hlist,
            hcons(123, hcons("abc", hcons(true, hcons(['a', 'b'], HNil))))
        );
    }

    #[test]
    fn from_tuple_into_hlist() {
        type HListType = hlist_type!(i32, &'static str, bool);
        let tuple = (123, "abc", true);
        let expected = hlist!(123, "abc", true);

        let hlist = HListType::from(tuple);
        assert_eq!(hlist, expected);

        let hlist: HListType = tuple.into();
        assert_eq!(hlist, expected);
    }
}
