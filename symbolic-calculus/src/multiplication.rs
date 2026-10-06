use core::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus_macros::{
    Apply, Arity, ComparisonTokens, Display, Function, Output, RenameVariable, SetNamedConstValue,
    VariableNames,
};

use crate::util::Alternative;
use crate::{AlwaysConst, Derivative, EvaluatedDisplayData, FunctionDisplay, impl_ops_for};

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
#[function(operands = 2, operator = *)]
pub struct Multiplication<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Multiplication<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Multiplication { left, right }
    }
}

#[ops_bounds(
    // l'
    L: Derivative<V> + Clone,
    // r'
    R: Derivative<V> + Clone,
    // l' * r + l * r'
    L::Derivative * R + L * R::Derivative,
)]
impl<L, R, const V: usize> Derivative<V> for Multiplication<L, R> {
    type Derivative = ops_type!(L::Derivative * R + L * R::Derivative);

    fn derivative(this: &Self) -> Self::Derivative {
        /*
           mul(x) = l(x) . r(x)
           mul'(x) = l'(x).r(x) + l(x).r'(x)
        */
        let l = this.left.clone();
        let r = this.right.clone();

        let l_der = l.derivative();
        let r_der = r.derivative();

        l_der * r + l * r_der
    }
}

impl<L, R> FunctionDisplay for Multiplication<L, R>
where
    L: FunctionDisplay,
    R: FunctionDisplay,
{
    const PRECEDENCE: u8 = 1;

    fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        let left = self.left.evaluated_display_data();
        let right = self.right.evaluated_display_data();

        match (left.always_const, right.always_const) {
            // 0 * y = 0, x * 1 = x
            (Some(AlwaysConst::Zero), _) | (_, Some(AlwaysConst::One)) => {
                left.map(Alternative::first)
            }
            // x * 0 = 0, 1 * y = y
            (_, Some(AlwaysConst::Zero)) | (Some(AlwaysConst::One), _) => {
                right.map(Alternative::second)
            }
            // x * y
            (_, _) => Self::default_binop_evaluated_display_data(
                left.display_data,
                '*',
                right.display_data,
            )
            .map(Alternative::third_last),
        }
    }
}

impl_ops_for!(Multiplication<L, R>);

#[macro_export]
macro_rules! impl_mul_for {
    ($name:ident $($generics:tt)*) => {
        macro_rules! callback {
            ([$$([$$($$decl:tt)+])*] [$$($$use:tt)*] [$type_var:ident $$($$tt:tt)*]) => {
                impl< $$($$($$decl)+,)* $$type_var> ::core::ops::Mul<$$type_var> for $name< $$($$use)* > {
                    type Output = $crate::Multiplication<Self, $type_var>;

                    fn mul(self, rhs: $type_var) -> Self::Output {
                        $crate::Multiplication::new(self, rhs)
                    }
                }
            };
        }

        $crate::__process_generics!([$($generics)*] [F, G, H] callback!);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::FixedType;
    use crate::{
        Addition, Division, Negation, Squared, StaticOne, StaticZero, Subtraction, TypedConst,
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
            assert_eq!((ZERO * ZERO).to_string(), "0");
            assert_eq!((ZERO * ONE).to_string(), "0");
            assert_eq!((ZERO * VAR).to_string(), "0");

            assert_eq!((ONE * ZERO).to_string(), "0");
            assert_eq!((ONE * ONE).to_string(), "1");
            assert_eq!((ONE * VAR).to_string(), "x");

            assert_eq!((VAR * ZERO).to_string(), "0");
            assert_eq!((VAR * ONE).to_string(), "x");
            assert_eq!((VAR * VAR).to_string(), "x * x");
        }

        #[test]
        fn precedence() {
            assert_eq!((NEG * NEG).to_string(), "-7 * -7");
            assert_eq!((ADD * ADD).to_string(), "(7 + x) * (7 + x)");
            assert_eq!((SUB * SUB).to_string(), "(x - 7) * (x - 7)");
            assert_eq!((MUL * MUL).to_string(), "7 * x * (7 * x)");
            assert_eq!((DIV * DIV).to_string(), "x / 7 * (x / 7)");
            assert_eq!((SQR * SQR).to_string(), "7² * 7²");
        }
    }
}
