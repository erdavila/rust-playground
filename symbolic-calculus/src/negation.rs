use core::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::util::{Alternative, OperationAssociativity};
use crate::{
    AlwaysConst, Apply, Derivative, DisplayData, EvaluatedDisplayData, FunctionDisplay, Output,
    StaticZero, impl_ops_for,
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
pub struct Negation<F>(pub F);

impl<F> Negation<F> {
    pub const fn new(func: F) -> Self {
        Negation(func)
    }

    pub fn evaluated_display_data_with_operand(
        operand: DisplayData<impl Display>,
    ) -> EvaluatedDisplayData<impl Display>
    where
        Self: FunctionDisplay,
    {
        Self::default_unop_evaluated_display_data(
            "-",
            operand,
            "",
            OperationAssociativity::LeftToRight,
        )
    }
}

#[ops_bounds(-F::Output)]
impl<F, I> Output<I> for Negation<F>
where
    F: Output<I>,
{
    type Output = ops_type!(-F::Output);
}

#[ops_bounds(-F::Output = Self::Output)]
impl<F, I> Apply<I> for Negation<F>
where
    Self: Output<I>,
    F: Apply<I>,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        -this.0.apply(input)
    }
}

#[ops_bounds(-F::Derivative)]
impl<F, const V: usize> Derivative<V> for Negation<F>
where
    F: Derivative<V>,
{
    type Derivative = ops_type!(-F::Derivative);

    fn derivative(this: &Self) -> Self::Derivative {
        /*
            neg(x) = -f(x)
                   = g(f(x))
            neg'(x) = g'(f(x)).f'(x)
                    = -f'(x)

            g(x) = -x
            g'(x) = -1
        */
        -this.0.derivative()
    }
}

impl<F> FunctionDisplay for Negation<F>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = 2;

    fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        let func = self.0.evaluated_display_data();
        match func.always_const {
            // -0 = 0
            Some(AlwaysConst::Zero) => StaticZero::<Self>::new()
                .evaluated_display_data()
                .map(Alternative::first),
            // -1, -x
            Some(AlwaysConst::One) | None => {
                Self::evaluated_display_data_with_operand(func.display_data)
                    .map(Alternative::second_last)
            }
        }
    }
}

impl_ops_for!(Negation<F>);

#[macro_export]
macro_rules! impl_neg_for {
    ($name:ident $($generics:tt)*) => {
        macro_rules! callback {
            ([$$([$$($$decl:tt)+])*] [$$($$use:tt)*]) => {
                impl< $$($$($$decl)+),*> ::core::ops::Neg for $name< $$($$use)* > {
                    type Output = $crate::Negation<Self>;

                    fn neg(self) -> Self::Output {
                        $crate::Negation::new(self)
                    }
                }
            };
        }

        $crate::__parse_generics!([$($generics)*] callback!);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    mod function_display {
        use super::*;
        use crate::util::FixedType;
        use crate::{
            Addition, Division, Multiplication, Squared, StaticOne, Subtraction, TypedConst,
            Variable,
        };

        type OtherConstType = TypedConst<f64>;
        type VariableType = Variable<0, &'static str>;

        const ZERO: StaticZero<FixedType<f64>> = StaticZero::new();
        const ONE: StaticOne<FixedType<f64>> = StaticOne::new();
        const OTHER: OtherConstType = TypedConst::new(7.0);

        const VAR: VariableType = Variable::with_name0("x");
        const NEG: Negation<OtherConstType> = Negation::new(OTHER);
        const ADD: Addition<OtherConstType, VariableType> = Addition::new(OTHER, VAR);
        const SUB: Subtraction<VariableType, OtherConstType> = Subtraction::new(VAR, OTHER);
        const MUL: Multiplication<OtherConstType, VariableType> = Multiplication::new(OTHER, VAR);
        const DIV: Division<VariableType, OtherConstType> = Division::new(VAR, OTHER);
        const SQR: Squared<OtherConstType> = Squared::new(OTHER);

        #[test]
        fn simplification() {
            assert_eq!((-ZERO).to_string(), "0");
            assert_eq!((-ONE).to_string(), "-1");
            assert_eq!((-VAR).to_string(), "-x");
        }

        #[test]
        fn precedence() {
            assert_eq!((-VAR).to_string(), "-x");
            assert_eq!((-NEG).to_string(), "--7");
            assert_eq!((-ADD).to_string(), "-(7 + x)");
            assert_eq!((-SUB).to_string(), "-(x - 7)");
            assert_eq!((-MUL).to_string(), "-(7 * x)");
            assert_eq!((-DIV).to_string(), "-(x / 7)");
            assert_eq!((-SQR).to_string(), "-7²");
        }
    }
}
