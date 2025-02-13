use crate::{HCons, HList, HNil};

/// Implements the handling of an [`HList`] element type.
///
/// # Examples
///
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
pub trait ForEach<Input> {
    /// Executes a handler on each element of the homogeneous list.
    fn for_each(&mut self, value: Input);
}

/// Implements interation over an [`HList`].
///
/// See [`ForEach`](crate::ForEach) for more information.
pub trait Over<Input>
where
    Input: HList,
{
    /// Iterates over the `input` argument.
    fn for_each_over(this: &mut Self, input: Input);
}

impl<F> Over<HNil> for F {
    fn for_each_over(_: &mut Self, _: HNil) {}
}

impl<H, T, F> Over<HCons<H, T>> for F
where
    T: HList,
    F: ForEach<H> + Over<T>,
{
    fn for_each_over(this: &mut Self, input: HCons<H, T>) {
        ForEach::for_each(this, input.head);
        Over::for_each_over(this, input.tail);
    }
}

#[cfg(test)]
mod tests {
    use crate::hlist;

    use super::*;

    #[test]
    fn for_each() {
        let mut log = vec![];

        let hlist = hlist!(123i32, "abc", true);

        hlist.for_each({
            struct F<'s>(&'s mut Vec<String>);
            impl ForEach<i32> for F<'_> {
                fn for_each(&mut self, value: i32) {
                    self.0.push(format!("i32: {value:?}"));
                }
            }
            impl ForEach<&str> for F<'_> {
                fn for_each(&mut self, value: &str) {
                    self.0.push(format!("&str: {value:?}"));
                }
            }
            impl ForEach<bool> for F<'_> {
                fn for_each(&mut self, value: bool) {
                    self.0.push(format!("bool: {value:?}"));
                }
            }
            F(&mut log)
        });

        assert_eq!(log, vec!["i32: 123", "&str: \"abc\"", "bool: true"]);
    }
}
