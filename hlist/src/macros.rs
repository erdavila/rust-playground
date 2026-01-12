/// Creates an [`HList`](crate::HList) with any number of elements.
///
/// # Example
///
/// ```
/// use hlist::{hcons, hlist, hnil};
///
/// let hlist = hlist!(42, '@');
///
/// assert_eq!(hlist, hcons(42usize, hcons('@', hnil())));
/// ```
#[macro_export]
macro_rules! hlist {
    () => {
        $crate::HNil
    };
    ( $head:expr $( , $tail:expr )* $(,)? ) => {
        $crate::hcons($head, $crate::hlist!( $( $tail ),* ))
    };
}

/// Makes an specific [`HList`](crate::HList) type for the given elements types.
///
/// # Example
///
/// ```
/// use hlist::{hcons, hlist, hnil, hlist_type};
///
/// let hlist: hlist_type!(usize, char) = hlist!(42, '@');
///
/// assert_eq!(hlist, hcons(42usize, hcons('@', hnil())));
/// ```
#[macro_export]
macro_rules! hlist_type {
    () => {
        $crate::HNil
    };
    ( $head:ty $( , $tail:ty )* ) => {
        $crate::HCons<$head, $crate::hlist_type!( $( $tail ),* )>
    };
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::{HCons, HNil, hcons, hnil};

    #[test]
    fn hlist() {
        let hlist = hlist!();
        assert_eq!(hlist, hnil());

        let hlist = hlist!(123);
        assert_eq!(hlist, hcons(123, hnil()));

        let hlist = hlist!(123,);
        assert_eq!(hlist, hcons(123, hnil()));

        let hlist = hlist!(123, "abc");
        assert_eq!(hlist, hcons(123, hcons("abc", hnil())));

        let hlist = hlist!(123, "abc",);
        assert_eq!(hlist, hcons(123, hcons("abc", hnil())));

        let hlist = hlist!(123, "abc", true);
        assert_eq!(hlist, hcons(123, hcons("abc", hcons(true, hnil()))));

        let hlist = hlist!(123, "abc", true,);
        assert_eq!(hlist, hcons(123, hcons("abc", hcons(true, hnil()))));
    }

    #[test]
    fn hlist_type() {
        assert_eq!(TypeId::of::<hlist_type!()>(), TypeId::of::<HNil>());
        assert_eq!(
            TypeId::of::<hlist_type!(i32)>(),
            TypeId::of::<HCons<i32, HNil>>()
        );
        assert_eq!(
            TypeId::of::<hlist_type!(i32, &str)>(),
            TypeId::of::<HCons<i32, HCons<&str, HNil>>>()
        );
        assert_eq!(
            TypeId::of::<hlist_type!(i32, &str, bool)>(),
            TypeId::of::<HCons<i32, HCons<&str, HCons<bool, HNil>>>>()
        );
    }
}
