use core::fmt::Display;
use core::marker::PhantomData;

use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::{
    Apply, Derivative, DisplayData, EvaluatedDisplayData, FunctionDisplay, Output, impl_ops_for,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Function,
    RenameVariable,
    VariableNames,
    SetNamedConstValue,
    Arity,
    ComparisonTokens,
    Display,
)]
#[function(operands = 1)]
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

impl<F, G, I> Output<I> for Convert<F, G>
where
    F: Output<I>,
    G: Output<I>,
    F::Output: Into<G::Output>,
{
    type Output = G::Output;
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

impl<F, G, const V: usize> Derivative<V> for Convert<F, G>
where
    F: Derivative<V>,
{
    type Derivative = Convert<F::Derivative, G>;

    fn derivative(this: &Self) -> Self::Derivative {
        Convert::new(this.func.derivative())
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

impl_ops_for!(Convert<F, G>);
