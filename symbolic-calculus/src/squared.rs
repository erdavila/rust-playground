use core::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::util::{Alternative, OperationAssociativity, Sqr};
use crate::{
    AlwaysConst, Apply, Derivative, EvaluatedDisplayData, FunctionDisplay, Output, StaticTwo,
    impl_ops_for,
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
pub struct Squared<F>(pub F);

impl<F> Squared<F> {
    pub const fn new(func: F) -> Self {
        Squared(func)
    }
}

#[ops_bounds(F::Output ^ 2)]
impl<F, I> Output<I> for Squared<F>
where
    F: Output<I>,
{
    type Output = ops_type!(F::Output ^ 2);
}

#[ops_bounds(F::Output ^ 2 = Self::Output)]
impl<F, I> Apply<I> for Squared<F>
where
    Self: Output<I>,
    F: Apply<I>,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        this.0.apply(input).sqr()
    }
}

#[ops_bounds(
    F: Derivative<V> + Clone,
    StaticTwo<F> * F * F::Derivative,
)]
impl<F, const V: usize> Derivative<V> for Squared<F> {
    type Derivative = ops_type!(StaticTwo<F> * F * F::Derivative);

    fn derivative(this: &Self) -> Self::Derivative {
        /*
            f(x) = g(h(x))
                 = h(x)²
            f'(x) = g'(h(x)).h'(x)
                  = 2.h(x).h'(x)

            g(x) = x²
            g'(x) = 2x
        */
        StaticTwo::new() * this.0.clone() * this.0.derivative::<V>()
    }
}

impl<F> FunctionDisplay for Squared<F>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = 2;

    fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        let func = self.0.evaluated_display_data();
        match func.always_const {
            // 0² = 0, 1² = 1
            Some(AlwaysConst::Zero | AlwaysConst::One) => func.map(Alternative::first),
            // x²
            None => Self::default_unop_evaluated_display_data(
                "",
                func.display_data,
                "²",
                OperationAssociativity::RightToLeft,
            )
            .map(Alternative::second_last),
        }
    }
}

impl_ops_for!(Squared<F>);

#[macro_export]
macro_rules! impl_sqr_for {
    ($name:ident $($generics:tt)*) => {
        macro_rules! callback {
            ([$$([$$($$decl:tt)+])*] [$$($$use:tt)*]) => {
                impl< $$($$($$decl)+),*> $crate::util::Sqr for $name< $$($$use)* > {
                    type Output = $crate::Squared<Self>;

                    fn sqr(self) -> Self::Output {
                        $crate::Squared::new(self)
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
            Addition, Division, Multiplication, Negation, Squared, StaticOne, StaticZero,
            Subtraction, TypedConst, Variable,
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
            assert_eq!(ZERO.sqr().to_string(), "0");
            assert_eq!(ONE.sqr().to_string(), "1");
            assert_eq!(VAR.sqr().to_string(), "x²");
        }

        #[test]
        fn precedence() {
            assert_eq!(VAR.sqr().to_string(), "x²");
            assert_eq!(NEG.sqr().to_string(), "(-7)²");
            assert_eq!(ADD.sqr().to_string(), "(7 + x)²");
            assert_eq!(SUB.sqr().to_string(), "(x - 7)²");
            assert_eq!(MUL.sqr().to_string(), "(7 * x)²");
            assert_eq!(DIV.sqr().to_string(), "(x / 7)²");
            assert_eq!(SQR.sqr().to_string(), "(7²)²");
        }
    }
}
