use crate::{get::Get, ForEachOver, MapOver};

/// The abstract representation of heterogeneous lists.
pub trait HList {
    /// The length of the heterogeneous list.
    const LENGTH: usize;

    /// The corresponding [`HList`] with references to the elements.
    ///
    /// It is the return type of [`HList::as_ref`].
    type AsRef<'a>: HList
    where
        Self: 'a;

    /// The corresponding [`HList`] with mutable references to the elements.
    ///
    /// It is the return type of [`HList::as_mut`].
    type AsMut<'a>: HList
    where
        Self: 'a;

    /// Provides the length of the heterogeneous list.
    fn len(&self) -> usize;

    /// Checks if the heterogeneous list is [`HNil`]
    fn is_empty(&self) -> bool {
        Self::LENGTH == 0
    }

    /// Gets an [`HList`] with references to the elements.
    fn as_ref(&self) -> Self::AsRef<'_>;

    /// Gets an [`HList`] with mutable references to the elements.
    fn as_mut(&mut self) -> Self::AsMut<'_>;

    /// Gets a reference to the element at index `N`.
    ///
    /// # Example
    ///
    /// ```
    /// use hlist::{HList, hlist};
    ///
    /// let hlist = hlist!(123, "abc", true);
    ///
    /// assert_eq!(hlist.get::<0>(), &123);
    /// assert_eq!(hlist.get::<1>(), &"abc");
    /// assert_eq!(hlist.get::<2>(), &true);
    /// ```
    fn get<const N: usize>(&self) -> &<Self as Get<N>>::Output
    where
        Self: Get<N>,
    {
        <Self as Get<N>>::get(self)
    }

    /// Gets a mutable reference to the element at index `N`.
    ///
    /// # Example
    ///
    /// ```
    /// use hlist::{HList, hlist};
    ///
    /// let mut hlist = hlist!(123, "abc", true);
    /// *hlist.get_mut::<0>() = 456;
    /// *hlist.get_mut::<1>() = "def";
    /// *hlist.get_mut::<2>() = false;
    ///
    /// assert_eq!(hlist, hlist!(456, "def", false));
    fn get_mut<const N: usize>(&mut self) -> &mut <Self as Get<N>>::Output
    where
        Self: Get<N>,
    {
        <Self as Get<N>>::get_mut(self)
    }

    /// Maps the elements of the heterogeneous list.
    ///
    /// See [`Map`](crate::Map) for more information.
    ///
    /// # Example
    /// ```
    /// use hlist::{HList, hlist, Map};
    ///
    /// let hlist = hlist!(123i32, true);
    ///
    /// let hlist = hlist.map({
    ///     struct M;
    ///     impl Map<i32> for M {
    ///         type Output = String;
    ///         fn map(&mut self, value: i32) -> Self::Output {
    ///             value.to_string()
    ///         }
    ///     }
    ///     impl Map<bool> for M {
    ///         type Output = bool;
    ///         fn map(&mut self, value: bool) -> Self::Output {
    ///             !value
    ///         }
    ///     }
    ///     M
    /// });
    /// assert_eq!(hlist, hlist!(String::from("123"), false));
    /// ```
    fn map<M>(self, mut m: M) -> <M as MapOver<Self>>::Output
    where
        Self: Sized,
        M: MapOver<Self>,
    {
        m.map_over(self)
    }

    /// Executes a handler on each element of the heterogeneous list.
    ///
    /// See [`ForEach`](crate::ForEach) for more information.
    ///
    /// # Example
    /// ```
    /// use hlist::{hlist, HList, ForEach};
    ///
    /// struct State {
    ///     count: usize
    /// }
    /// let mut state = State { count: 0 };
    ///
    /// let hlist = hlist!(42usize, "xyz");
    /// hlist.for_each({
    ///     struct F<'a>(&'a mut State);
    ///     impl ForEach<usize> for F<'_> {
    ///         fn for_each(&mut self, value: usize) {
    ///             self.0.count += value;
    ///         }
    ///     }
    ///     impl ForEach<&'static str> for F<'_> {
    ///         fn for_each(&mut self, value: &'static str) {
    ///             self.0.count += value.len();
    ///         }
    ///     }
    ///     F(&mut state)
    /// });
    ///
    /// assert_eq!(state.count, 45);
    /// ```
    fn for_each<F>(self, mut f: F)
    where
        Self: Sized,
        F: ForEachOver<Self>,
    {
        f.for_each_over(self);
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

    type AsRef<'a> = HNil;

    type AsMut<'a> = HNil;

    fn len(&self) -> usize {
        Self::LENGTH
    }

    fn as_ref(&self) -> Self::AsRef<'_> {
        HNil
    }

    fn as_mut(&mut self) -> Self::AsMut<'_> {
        HNil
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

    type AsRef<'a>
        = HCons<&'a H, T::AsRef<'a>>
    where
        Self: 'a;

    type AsMut<'a>
        = HCons<&'a mut H, T::AsMut<'a>>
    where
        Self: 'a;

    fn len(&self) -> usize {
        Self::LENGTH
    }

    fn as_ref(&self) -> Self::AsRef<'_> {
        HCons::new(&self.head, self.tail.as_ref())
    }

    fn as_mut(&mut self) -> Self::AsMut<'_> {
        HCons::new(&mut self.head, self.tail.as_mut())
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
    use crate::hlist;

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

    #[test]
    fn as_ref() {
        assert_eq!(hlist!().as_ref(), hlist!());
        assert_eq!(hlist!(123).as_ref(), hlist!(&123));
        assert_eq!(hlist!(123, "abc").as_ref(), hlist!(&123, &"abc"));
        assert_eq!(
            hlist!(123, "abc", true).as_ref(),
            hlist!(&123, &"abc", &true)
        );
    }

    #[test]
    fn as_mut() {
        let mut hlist = hlist!(123, "abc", true);

        let hl = hlist.as_mut();
        *hl.head = 456;
        *hl.tail.head = "def";
        *hl.tail.tail.head = false;

        assert_eq!(hlist, hlist!(456, "def", false));
    }
}
