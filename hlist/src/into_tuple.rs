use crate::tuples::Tuple;
use crate::{HList, hlist_type};

/// A trait implemented by [`HList`]s to be converted into tuples.
pub trait IntoTuple: HList {
    /// The corresponding tuple type.
    type Tuple: Tuple;

    /// Turns the [`HList`] into a tuple.
    fn into_tuple(self) -> Self::Tuple;
}

macro_rules! impl_into_tuple_trait {
    ($( $type_arg:ident )* ; $tuple_closure:expr_2021 ) => {
        impl< $( $type_arg ,)* > IntoTuple for hlist_type!( $( $type_arg ),* ) {
            type Tuple = ( $( $type_arg ,)* );

            fn into_tuple(self) -> Self::Tuple {
                let tuple_closure: fn(Self) -> _ = $tuple_closure;
                (tuple_closure)(self)
            }
        }

        impl< $( $type_arg ,)* > From<hlist_type!( $( $type_arg ),* )> for ( $( $type_arg ,)* ) {
            fn from(hlist: hlist_type!( $( $type_arg ),* )) -> Self {
                hlist.into_tuple()
            }
        }
    };
}

impl_into_tuple_trait!(; |_| ());
impl_into_tuple_trait!(A0 ; |this| (this.head,));
impl_into_tuple_trait!(A0 A1 ; |this| (this.head, this.tail.head));
impl_into_tuple_trait!(A0 A1 A2 ; |this| (this.head, this.tail.head, this.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.head));
impl_into_tuple_trait!(A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 ; |this| (this.head, this.tail.head, this.tail.tail.head, this.tail.tail.tail.head, this.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.head, this.tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.tail.head));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hlist;

    #[test]
    fn into_tuple() {
        #[expect(clippy::let_unit_value)]
        let tuple = hlist!().into_tuple();
        assert_eq!(tuple, ());

        let tuple = hlist!(123).into_tuple();
        assert_eq!(tuple, (123,));

        let tuple = hlist!(123, "abc").into_tuple();
        assert_eq!(tuple, (123, "abc"));

        let tuple = hlist!(123, "abc", true).into_tuple();
        assert_eq!(tuple, (123, "abc", true));

        let tuple = hlist!(123, "abc", true, ['a', 'b']).into_tuple();
        assert_eq!(tuple, (123, "abc", true, ['a', 'b']));
    }

    #[test]
    fn from_hlist_into_tuple() {
        type TupleType = (i32, &'static str, bool);
        let hlist = hlist!(123, "abc", true);
        let expected = (123, "abc", true);

        let tuple = TupleType::from(hlist);
        assert_eq!(tuple, expected);

        let tuple: TupleType = hlist.into();
        assert_eq!(tuple, expected);
    }
}
