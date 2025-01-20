#![warn(missing_docs)]

//! Provides heterogeneous lists.

/// The abstract representation of heterogeneous lists.
pub trait HList {
    /// Provides the length of the heterogeneous list.
    fn len(&self) -> usize;

    /// Checks if the heterogeneous list is [`HNil`]
    fn is_empty(&self) -> bool;
}

/// The empty [`HList`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HNil;
impl HList for HNil {
    fn len(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
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
}
impl<H, T> HList for HCons<H, T>
where
    T: HList,
{
    fn len(&self) -> usize {
        1 + self.tail.len()
    }

    fn is_empty(&self) -> bool {
        false
    }
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
    fn len_and_is_empty() {
        let hlist = HNil;
        assert_eq!(hlist.len(), 0);
        assert!(hlist.is_empty());

        let hlist = HCons::new(123, HNil);
        assert_eq!(hlist.len(), 1);
        assert!(!hlist.is_empty());

        let hlist = HCons::new(123, HCons::new("abc", HNil));
        assert_eq!(hlist.len(), 2);
        assert!(!hlist.is_empty());

        let hlist = HCons::new(123, HCons::new("abc", HCons::new(true, HNil)));
        assert_eq!(hlist.len(), 3);
        assert!(!hlist.is_empty());
    }
}
