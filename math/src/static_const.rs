use core::fmt::{self, Display};
use core::iter;
use core::marker::PhantomData;

use crate::{
    AlwaysConst, Apply, ComparisonTokens, Derivative, EvaluatedDisplayData, Function,
    FunctionDisplay, Output, VariableNames, comparison, comparison_id, impl_ops_for,
};

macro_rules! define_const {
    ($name:ident $trait:ident $const:ident $display:literal $always_const:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        impl<F> Function for $name<F> {}

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

        impl<F, const V: usize> Derivative<V> for $name<F> {
            type Derivative = StaticZero<F>;

            fn derivative(_this: &Self) -> Self::Derivative {
                StaticZero::new()
            }
        }

        impl<F> ComparisonTokens for $name<F> {
            fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
                comparison::TokensBuilder::new(comparison_id!($name)).build()
            }
        }

        impl<F> VariableNames for $name<F> {
            fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
                iter::empty()
            }
        }

        impl<F> FunctionDisplay for $name<F> {
            const PRECEDENCE: u8 = 2;

            #[expect(refining_impl_trait_reachable)]
            fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display + 'static> {
                Self::maybe_const_evaluated_display_data($always_const, $display)
            }
        }

        impl<F> Display for $name<F> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                todo!("display");
                $display.fmt(f)
            }
        }

        impl_ops_for!($name<F>);
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
