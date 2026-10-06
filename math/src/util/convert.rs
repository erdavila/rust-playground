use core::fmt::{self, Display};
use core::marker::PhantomData;

use crate::{
    Apply, ComparisonTokens, Derivative, DisplayData, EvaluatedDisplayData, Function,
    FunctionDisplay, Output, comparison, impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Convert<F, G> {
    func: F,
    phantom: PhantomData<G>,
}

impl<F, G> Convert<F, G> {
    pub const fn new(func: F) -> Self {
        Convert {
            func,
            phantom: PhantomData,
        }
    }
}

impl<F, G> Function for Convert<F, G> {}

impl<F, G, I> Output<I> for Convert<F, G>
where
    F: Output<I>,
    G: Output<I>,
    F::Output: Into<G::Output>,
{
    type Output = <G as Output<I>>::Output;
}

impl<F, G, I> Apply<I> for Convert<F, G>
where
    Self: Output<I>,
    F: Apply<I>,
    F::Output: Into<Self::Output>,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        this.func.apply(input).into()
    }
}

impl<F, G, const V: usize> Derivative<V> for Convert<F, G> {
    type Derivative = Self;

    fn derivative(this: &Self) -> Self::Derivative {
        todo!()
    }
}

impl<F, G> FunctionDisplay for Convert<F, G>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = F::PRECEDENCE;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        EvaluatedDisplayData {
            always_const: todo!(),
            display_data: DisplayData {
                precedence: todo!(),
                display: 0,
            },
        }
    }
}

impl<F, G> ComparisonTokens for Convert<F, G> {
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        todo!();
        [].into_iter()
    }
}

impl<F, G> Display for Convert<F, G>
where
    F: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        // TODO: review
        self.func.fmt(f)
    }
}

impl_ops_for!(Convert<F, G>);
