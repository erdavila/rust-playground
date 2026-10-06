use core::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus_macros::{
    Apply, Arity, ComparisonTokens, Display, Function, Output, RenameVariable, SetNamedConstValue,
    VariableNames,
};

use crate::util::UnaryOperation;
use crate::{
    AlwaysConst, Derivative, EvaluatedDisplayData, FunctionDisplay, Negation, StaticZero,
    display_enum, impl_ops_for,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Function,
    Output,
    Apply,
    RenameVariable,
    VariableNames,
    SetNamedConstValue,
    Arity,
    ComparisonTokens,
    Display,
)]
#[function(operands = 2, operator = -)]
pub struct Subtraction<L, R> {
    pub left: L,
    pub right: R,
}
impl<L, R> Subtraction<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

#[ops_bounds(
    L: Derivative<V>,
    R: Derivative<V>,
    L::Derivative - R::Derivative,
)]
impl<L, R, const V: usize> Derivative<V> for Subtraction<L, R> {
    type Derivative = ops_type!(L::Derivative - R::Derivative);

    fn derivative(this: &Self) -> Self::Derivative {
        /*
           f(x) = g(x) - h(x)
           f'(x) = g'(x) - h'(x)
        */
        this.left.derivative() - this.right.derivative()
    }
}

impl<L, R> FunctionDisplay for Subtraction<L, R>
where
    L: FunctionDisplay,
    R: FunctionDisplay,
{
    const PRECEDENCE: u8 = 0;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        display_enum! {
            Case {
                Zero,
                Left,
                NegativeRight,
                NonSimplified,
            }
        }

        let left = self.left.evaluate_display_data();
        let right = self.right.evaluate_display_data();

        match (left.always_const, right.always_const) {
            // x - 0 = x
            (_, Some(AlwaysConst::Zero)) => left.map(Case::Left),
            // 0 - y = -y
            (Some(AlwaysConst::Zero), _) => {
                Negation::<R>::unop_evaluated_display_data(right.display_data)
                    .map(Case::NegativeRight)
            }
            // 1 - 1 = 0
            (Some(AlwaysConst::One), Some(AlwaysConst::One)) => StaticZero::<Self>::new()
                .evaluate_display_data()
                .map(Case::Zero),
            // x - y
            (_, _) => {
                Self::full_binop_evaluated_display_data(left.display_data, '-', right.display_data)
                    .map(Case::NonSimplified)
            }
        }
    }
}

impl_ops_for!(Subtraction<L, R>);

#[macro_export]
macro_rules! impl_sub_for {
    ($name:ident $($generics:tt)*) => {
        $crate::__parse_generics! { [$($generics)*]
            ([$$($$decl:tt)*] [$$($$use:tt)*]) => {
                impl< $$($$decl)* __F> core::ops::Sub<__F> for $name< $$($$use)* > {
                    type Output = $crate::Subtraction<Self, __F>;

                    fn sub(self, rhs: __F) -> Self::Output {
                        $crate::Subtraction::new(self, rhs)
                    }
                }
            };
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::FixedType;
    use crate::{
        Addition, Division, Multiplication, Negation, Squared, StaticOne, Subtraction, TypedConst,
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

    mod function_display {
        use super::*;

        #[test]
        fn simplification() {
            assert_eq!((ZERO - ZERO).to_string(), "0");
            assert_eq!((ZERO - ONE).to_string(), "-1");
            assert_eq!((ZERO - VAR).to_string(), "-x");

            assert_eq!((ONE - ZERO).to_string(), "1");
            assert_eq!((ONE - ONE).to_string(), "0");
            assert_eq!((ONE - VAR).to_string(), "1 - x");

            assert_eq!((VAR - ZERO).to_string(), "x");
            assert_eq!((VAR - ONE).to_string(), "x - 1");
            assert_eq!((VAR - VAR).to_string(), "x - x");
        }

        #[test]
        fn precedence() {
            assert_eq!((NEG - NEG).to_string(), "-7 - -7");
            assert_eq!((ADD - ADD).to_string(), "7 + x - (7 + x)");
            assert_eq!((SUB - SUB).to_string(), "x - 7 - (x - 7)");
            assert_eq!((MUL - MUL).to_string(), "7 * x - 7 * x");
            assert_eq!((DIV - DIV).to_string(), "x / 7 - x / 7");
            assert_eq!((SQR - SQR).to_string(), "7² - 7²");
        }
    }
}
