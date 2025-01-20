#![warn(missing_docs)]

//! Provides heterogeneous lists.

/// The abstract representation of heterogeneous lists.
pub trait HList {
    /// The length of the heterogeneous list.
    const LENGTH: usize;

    /// Provides the length of the heterogeneous list.
    fn len(&self) -> usize;

    /// Checks if the heterogeneous list is [`HNil`]
    fn is_empty(&self) -> bool {
        Self::LENGTH == 0
    }
}

/// The empty [`HList`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HNil;
impl HNil {
    /// Provides the length of the heterogeneous list in a `const` context.
    #[must_use]
    pub const fn len(&self) -> usize {
        Self::LENGTH
    }

    /// Checks if the heterogeneous list is [`HNil`] in a `const` context.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        Self::LENGTH == 0
    }
}
impl HList for HNil {
    const LENGTH: usize = 0;

    fn len(&self) -> usize {
        Self::LENGTH
    }
}

/// An [`HList`] with `H` at its first position, and `T` as the rest of the list.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HCons<H, T>
where
    T: HList,
{
    /// The first value in the list.
    pub head: H,

    /// The remaining values in the list, which is itself an [`HList`].
    pub tail: T,
}
impl<H, T> HCons<H, T>
where
    T: HList,
{
    /// Creates a non-empty [`HList`].
    pub const fn new(head: H, tail: T) -> Self {
        HCons { head, tail }
    }

    /// Provides the length of the heterogeneous list in a `const` context.
    pub const fn len(&self) -> usize {
        Self::LENGTH
    }

    /// Checks if the heterogeneous list is [`HNil`] in a `const` context.
    pub const fn is_empty(&self) -> bool {
        Self::LENGTH == 0
    }
}
impl<H, T> HList for HCons<H, T>
where
    T: HList,
{
    const LENGTH: usize = 1 + T::LENGTH;

    fn len(&self) -> usize {
        Self::LENGTH
    }
}

/// Creates an empty [`HList`].
#[must_use]
pub const fn hnil() -> HNil {
    HNil
}

/// Creates a non-empty [`HList`].
///
/// It is an alias for [`HCons::new`].
pub const fn hcons<H, T>(head: H, tail: T) -> HCons<H, T>
where
    T: HList,
{
    HCons::new(head, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn new() {
        let hlist = HCons::new(123, HNil);
        assert_eq!(
            hlist,
            HCons {
                head: 123,
                tail: HNil,
            }
        );

        let hlist = HCons::new(123, HCons::new("abc", HNil));
        assert_eq!(
            hlist,
            HCons {
                head: 123,
                tail: HCons {
                    head: "abc",
                    tail: HNil
                }
            }
        );

        let hlist = HCons::new(123, HCons::new("abc", HCons::new(true, HNil)));
        assert_eq!(
            hlist,
            HCons {
                head: 123,
                tail: HCons {
                    head: "abc",
                    tail: HCons {
                        head: true,
                        tail: HNil
                    }
                }
            }
        );
    }

    #[test]
    fn const_len_and_is_empty() {
        let hlist = hnil();
        assert_eq!(hlist.len(), 0);
        assert!(hlist.is_empty());

        let hlist = hcons(123, HNil);
        assert_eq!(hlist.len(), 1);
        assert!(!hlist.is_empty());

        let hlist = hcons(123, hcons("abc", HNil));
        assert_eq!(hlist.len(), 2);
        assert!(!hlist.is_empty());

        let hlist = hcons(123, hcons("abc", hcons(true, HNil)));
        assert_eq!(hlist.len(), 3);
        assert!(!hlist.is_empty());
    }

    #[test]
    fn trait_len_and_is_empty() {
        fn create(hlist: impl HList) -> impl HList {
            hlist
        }

        let hlist = create(HNil);
        assert_eq!(hlist.len(), 0);
        assert!(hlist.is_empty());

        let hlist = create(HCons::new(123, HNil));
        assert_eq!(hlist.len(), 1);
        assert!(!hlist.is_empty());

        let hlist = create(HCons::new(123, HCons::new("abc", HNil)));
        assert_eq!(hlist.len(), 2);
        assert!(!hlist.is_empty());

        let hlist = create(HCons::new(123, HCons::new("abc", HCons::new(true, HNil))));
        assert_eq!(hlist.len(), 3);
        assert!(!hlist.is_empty());
    }
}
