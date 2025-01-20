use crate::{HCons, HList, HNil};

/// Trait for pushing a value to the back of an [`HList`].
pub trait PushBack<A>: HList {
    /// The resulting [`HList`] after pushing a value to the back.
    type Output: HList;

    /// Pushes a value to the back of the [`HList`].
    fn push_back(this: Self, value: A) -> Self::Output;
}

impl<A> PushBack<A> for HNil {
    type Output = HCons<A, HNil>;
    fn push_back(this: Self, value: A) -> Self::Output {
        HCons {
            head: value,
            tail: this,
        }
    }
}

impl<A, H, T> PushBack<A> for HCons<H, T>
where
    T: PushBack<A>,
{
    type Output = HCons<H, T::Output>;
    fn push_back(this: Self, value: A) -> Self::Output {
        HCons {
            head: this.head,
            tail: PushBack::push_back(this.tail, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{hlist, hnil};

    use super::*;

    #[test]
    fn push_back() {
        let hlist = hnil().push_back(123).push_back("abc").push_back(true);
        assert_eq!(hlist, hlist!(123, "abc", true));
    }
}
