use core::fmt::Display;
use core::marker::PhantomData;

use symbolic_calculus_macros::{
    Arity, Derivative, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::{
    AlwaysConst, Apply, ComparisonTokens, EvaluatedDisplayData, FunctionDisplay, Output,
    ValuedConst, comparison, comparison_id, impl_ops_for,
};

macro_rules! define_const {
    ($name:ident $trait:ident $const:ident $display:literal $always_const:expr) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Function,
            Derivative,
            RenameVariable,
            VariableNames,
            SetNamedConstValue,
            Arity,
            Display,
        )]
        #[function(const, type_function = F)]
        pub struct $name<F>(PhantomData<F>);

        impl<F> $name<F> {
            pub const fn new() -> Self {
                $name(PhantomData)
            }
        }

        impl<F> Default for $name<F> {
            fn default() -> Self {
                $name::new()
            }
        }

        impl<F, I> Output<I> for $name<F>
        where
            F: Output<I>,
        {
            type Output = F::Output;
        }

        impl<F, I> Apply<I> for $name<F>
        where
            Self: Output<I>,
            Self::Output: $trait,
        {
            fn apply(_this: &Self, _input: I) -> Self::Output {
                Self::Output::$const
            }
        }

        impl<F> ComparisonTokens for $name<F> {
            fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
                comparison::TokensBuilder::new(comparison_id!($name)).build()
            }
        }

        impl<F> FunctionDisplay for $name<F> {
            const PRECEDENCE: u8 = 2;

            #[expect(refining_impl_trait_reachable)]
            fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display + 'static> {
                Self::maybe_const_evaluated_display_data($always_const, $display)
            }
        }

        impl_ops_for!($name<F>);

        impl<F> ValuedConst for $name<F> {}
    };
}

define_const!(StaticZero Zero ZERO 0 Some(AlwaysConst::Zero));
define_const!(StaticOne One ONE 1 Some(AlwaysConst::One));
define_const!(StaticTwo Two TWO 2 None);

pub trait Zero {
    const ZERO: Self;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}

pub trait One {
    const ONE: Self;
}
impl One for f64 {
    const ONE: Self = 1.0;
}

pub trait Two {
    const TWO: Self;
}
impl Two for f64 {
    const TWO: Self = 2.0;
}
