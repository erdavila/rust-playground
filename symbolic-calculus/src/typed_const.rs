use core::fmt::Display;

use symbolic_calculus_macros::{
    Arity, Derivative, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::comparison::{self, AnyEq};
use crate::util::FixedType;
use crate::{
    AlwaysConst, Apply, ComparisonTokens, EvaluatedDisplayData, FunctionDisplay, One, Output,
    StaticZero, ValuedConst, Zero, comparison_id, impl_ops_for,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Function,
    Derivative,
    RenameVariable,
    VariableNames,
    SetNamedConstValue,
    Arity,
    Display,
)]
#[function(const, type_function = FixedType<T>)]
pub struct TypedConst<T> {
    pub value: T,
}

impl<T> TypedConst<T> {
    pub const fn new(value: T) -> Self {
        TypedConst { value }
    }
}

impl<T, I> Output<I> for TypedConst<T> {
    type Output = T;
}

impl<T, I> Apply<I> for TypedConst<T>
where
    Self: Output<I, Output = T>,
    T: Clone,
{
    fn apply(this: &Self, _input: I) -> Self::Output {
        this.value.clone()
    }
}

impl<T> ComparisonTokens for TypedConst<T>
where
    T: AnyEq,
{
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        comparison::TokensBuilder::new(comparison_id!(TypedConst))
            .value(&self.value)
            .build()
    }
}

impl<T> FunctionDisplay for TypedConst<T>
where
    T: Zero + One + PartialEq + Display,
{
    const PRECEDENCE: u8 = 3;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        let always_const = if self.value == T::ZERO {
            Some(AlwaysConst::Zero)
        } else if self.value == T::ONE {
            Some(AlwaysConst::One)
        } else {
            None
        };

        Self::maybe_const_evaluated_display_data(always_const, &self.value)
    }
}

impl_ops_for!(TypedConst<T>);

impl<T> ValuedConst for TypedConst<T> {}
