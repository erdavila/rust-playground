use crate::{HCons, HList, HNil};

/// Implements the mapping of an [`HList`] element type.
///
/// # Examples
///
/// ```
/// use hlist::{hlist, HList, Map};
///
/// let hlist = hlist!(123i32, "abc", true);
/// let hlist = hlist.map({
///     struct M;
///     impl Map<i32> for M {
///         type Output = String;
///         fn map(&mut self, value: i32) -> Self::Output {
///             value.to_string()
///         }
///     }
///     impl Map<&'static str> for M {
///         type Output = usize;
///         fn map(&mut self, value: &'static str) -> Self::Output {
///             value.len()
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
///
/// assert_eq!(hlist, hlist!(String::from("123"), 3, false));
/// ```
///
/// With some state:
/// ```
/// use hlist::{hlist, HList, Map};
///
/// struct State {
///     count: usize
/// }
/// let mut state = State { count: 0 };
///
/// let hlist = hlist!(42usize, "xyz");
/// let hlist = hlist.map({
///     struct M<'a>(&'a mut State);
///     impl Map<usize> for M<'_> {
///         type Output = String;
///         fn map(&mut self, value: usize) -> Self::Output {
///             self.0.count += value;
///             value.to_string()
///         }
///     }
///     impl Map<&'static str> for M<'_> {
///         type Output = usize;
///         fn map(&mut self, value: &'static str) -> Self::Output {
///             self.0.count += value.len();
///             value.len()
///         }
///     }
///     M(&mut state)
/// });
///
/// assert_eq!(hlist, hlist!(String::from("42"), 3usize));
/// assert_eq!(state.count, 45);
/// ```
pub trait Map<Input> {
    /// The output type.
    type Output;

    /// Maps the input to the output.
    fn map(&mut self, value: Input) -> Self::Output;
}

/// Implements mapping over heterogeneous lists.
///
/// See [`Map`](crate::Map) for more information.
pub trait Over<Input>
where
    Input: HList,
{
    /// The output of mapping `Self` over the `Input`.
    type Output: HList;

    /// Maps over the `input` argument.
    fn map_over(&mut self, input: Input) -> Self::Output;
}

impl<M> Over<HNil> for M {
    type Output = HNil;

    fn map_over(&mut self, _: HNil) -> Self::Output {
        HNil
    }
}

impl<H, T, M> Over<HCons<H, T>> for M
where
    T: HList,
    M: Map<H> + Over<T>,
{
    type Output = HCons<<M as Map<H>>::Output, <M as Over<T>>::Output>;

    fn map_over(&mut self, input: HCons<H, T>) -> Self::Output {
        let head = self.map(input.head);
        let tail = self.map_over(input.tail);
        HCons::new(head, tail)
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist;

    use super::*;

    #[test]
    fn map() {
        let hlist = hlist!(123i32, "abc", true);

        let hlist = hlist.map({
            struct M;
            impl Map<i32> for M {
                type Output = String;
                fn map(&mut self, value: i32) -> Self::Output {
                    value.to_string()
                }
            }
            impl Map<&'static str> for M {
                type Output = usize;
                fn map(&mut self, value: &'static str) -> Self::Output {
                    value.len()
                }
            }
            impl Map<bool> for M {
                type Output = bool;
                fn map(&mut self, value: bool) -> Self::Output {
                    !value
                }
            }
            M
        });

        assert_eq!(hlist, hlist!(String::from("123"), 3, false));
    }
}
