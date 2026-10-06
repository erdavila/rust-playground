mod internal;

use core::fmt::{self, Display};

use crate::variable::internal::{DerivativeDispatch, RenameVariableDispatch};
use crate::{
    Apply, ComparisonTokens, Derivative, EvaluatedDisplayData, Function, FunctionDisplay, Output,
    RenameVariable, SelectInput, VariableNames, comparison, impl_ops_for,
};

trait DefaultVariableNameValue {
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultVariableName<const V: usize>;
impl DefaultVariableNameValue for DefaultVariableName<0> {
    const NAME: &'static str = "x";
}
impl DefaultVariableNameValue for DefaultVariableName<1> {
    const NAME: &'static str = "y";
}
impl DefaultVariableNameValue for DefaultVariableName<2> {
    const NAME: &'static str = "z";
}
impl<const V: usize> Display for DefaultVariableName<V>
where
    Self: DefaultVariableNameValue,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(Self::NAME)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable<const V: usize, N> {
    name: N,
}

impl<const V: usize, N> Variable<V, N> {
    pub const fn with_name(name: N) -> Self {
        Variable { name }
    }

    pub const fn name(&self) -> &N {
        &self.name
    }
}
impl<const V: usize> Variable<V, DefaultVariableName<V>> {
    #[must_use]
    pub const fn new() -> Self
    where
        DefaultVariableName<V>: Display,
    {
        Self::with_name(DefaultVariableName)
    }
}
impl Variable<0, DefaultVariableName<0>> {
    #[must_use]
    pub const fn new0() -> Self {
        Self::new()
    }
}
impl Variable<1, DefaultVariableName<1>> {
    #[must_use]
    pub const fn new1() -> Self {
        Self::new()
    }
}
impl Variable<2, DefaultVariableName<2>> {
    #[must_use]
    pub const fn new2() -> Self {
        Self::new()
    }
}
impl<N> Variable<0, N> {
    pub const fn with_name0(name: N) -> Self {
        Self::with_name(name)
    }
}
impl<N> Variable<1, N> {
    pub const fn with_name1(name: N) -> Self {
        Self::with_name(name)
    }
}
impl<N> Variable<2, N> {
    pub const fn with_name2(name: N) -> Self {
        Self::with_name(name)
    }
}

impl<const V: usize> Default for Variable<V, DefaultVariableName<V>>
where
    DefaultVariableName<V>: Display,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const V: usize, N> Function for Variable<V, N> {}

impl<const V: usize, N, I> Output<I> for Variable<V, N>
where
    I: SelectInput<V>,
{
    type Output = I::VariableOutput;
}

impl<const V: usize, N, I> Apply<I> for Variable<V, N>
where
    I: SelectInput<V>,
{
    fn apply(_this: &Self, input: I) -> Self::Output {
        input.select_input()
    }
}

impl<const V: usize, const V_DERIV: usize, N> Derivative<V_DERIV> for Variable<V, N>
where
    [(); (V == V_DERIV) as usize]:,
    Self: DerivativeDispatch<{ V == V_DERIV }>,
{
    type Derivative = <Self as DerivativeDispatch<{ V == V_DERIV }>>::Der;

    fn derivative(this: &Self) -> Self::Derivative {
        this.dispatch_derivative()
    }
}

impl<const V: usize, N, const V2: usize> RenameVariable<V2> for Variable<V, N>
where
    [(); (V == V2) as usize]:,
    Self: RenameVariableDispatch<{ V == V2 }>,
{
    type RenamedVar<T> = <Self as RenameVariableDispatch<{ V == V2 }>>::Ren<T>;

    fn rename_var_num<N2>(this: &Self, name: N2) -> Self::RenamedVar<N2> {
        this.dispatch_rename_var(name)
    }
}

impl<const V: usize, N> VariableNames for Variable<V, N>
where
    N: Display,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        let var_name: &dyn Display = &self.name;
        core::iter::once((V, var_name))
    }
}

impl<const V: usize, N> FunctionDisplay for Variable<V, N>
where
    N: Display,
{
    const PRECEDENCE: u8 = 3;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        Self::non_const_evaluated_display_data(&self.name)
    }
}

impl<const V: usize, N> ComparisonTokens for Variable<V, N> {
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        todo!();
        [].into_iter()
    }
}

impl<const V: usize, N> Display for Variable<V, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl_ops_for!(Variable<const V: usize, N>);

pub struct FullInput;
impl Function for FullInput {}
impl<I> Output<I> for FullInput {
    type Output = I;
}
impl<I> Apply<I> for FullInput {
    fn apply(this: &Self, input: I) -> Self::Output {
        todo!()
    }
}
