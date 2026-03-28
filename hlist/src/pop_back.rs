use crate::{HCons, HList, HNil};

/// Trait for popping a value from the back of an [`HList`].
pub trait PopBack: HList {
    /// The type of the popped value.
    type Element;

    /// The resulting [`HList`] after popping a value from the back.
    type ResultingHList: HList;

    /// Pops a value from the back of the [`HList`].
    fn pop_back(this: Self) -> (Self::Element, Self::ResultingHList);
}

impl<A> PopBack for HCons<A, HNil> {
    type Element = A;

    type ResultingHList = HNil;

    fn pop_back(this: Self) -> (Self::Element, Self::ResultingHList) {
        (this.head, HNil)
    }
}

impl<H1, H2, T> PopBack for HCons<H1, HCons<H2, T>>
where
    T: HList,
    HCons<H2, T>: PopBack,
{
    type Element = <HCons<H2, T> as PopBack>::Element;

    type ResultingHList = HCons<H1, <HCons<H2, T> as PopBack>::ResultingHList>;

    fn pop_back(this: Self) -> (Self::Element, Self::ResultingHList) {
        let (value, tail) = PopBack::pop_back(this.tail);
        let hlist = HCons {
            head: this.head,
            tail,
        };

        (value, hlist)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist;

    #[test]
    fn pop_back() {
        let hlist = hlist!(123, "abc", true);

        let (value, hlist) = hlist.pop_back();
        assert!(value);
        assert_eq!(hlist, hlist!(123, "abc"));

        let (value, hlist) = hlist.pop_back();
        assert_eq!(value, "abc");
        assert_eq!(hlist, hlist!(123));

        let (value, hlist) = hlist.pop_back();
        assert_eq!(value, 123);
        assert_eq!(hlist, hlist!());
    }
}
