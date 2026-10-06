use core::fmt::{self, Display};
use core::iter;

use crate::comparison::{self, AnyEq};
use crate::util::FixedType;
use crate::{
    AlwaysConst, Apply, ComparisonTokens, Derivative, EvaluatedDisplayData, Function,
    FunctionDisplay, One, Output, RenameVariable, StaticZero, VariableNames, Zero, comparison_id,
    impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypedConst<T> {
    pub value: T,
}

impl<T> TypedConst<T> {
    pub const fn new(value: T) -> Self {
        TypedConst { value }
    }
}

impl<T> Function for TypedConst<T> {}

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

impl<T, const V: usize> Derivative<V> for TypedConst<T> {
    type Derivative = StaticZero<FixedType<T>>;

    fn derivative(_this: &Self) -> Self::Derivative {
        StaticZero::new()
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

impl<T, const V: usize> RenameVariable<V> for TypedConst<T>
where
    Self: Clone,
{
    type RenamedVar<N> = Self;

    fn rename_var_num<N>(this: &Self, _name: N) -> Self::RenamedVar<N> {
        this.clone()
    }
}

impl<T> VariableNames for TypedConst<T> {
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        iter::empty()
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

impl<T: Display> Display for TypedConst<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        self.value.fmt(f)
    }
}

impl_ops_for!(TypedConst<T>);
