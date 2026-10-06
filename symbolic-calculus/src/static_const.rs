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

        impl<F> Clone for $name<F> {
            fn clone(&self) -> Self {
                Self::new()
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
            F::Output: $trait,
        {
            type Output = <F::Output as $trait>::Type;
        }

        impl<F, I> Apply<I> for $name<F>
        where
            Self: Output<I>,
            F: Output<I>,
            F::Output: $trait<Type = Self::Output>,
        {
            fn apply(_this: &Self, _input: I) -> Self::Output {
                <F::Output as $trait>::$const
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
            fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display + 'static> {
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
    type Type;
    const ZERO: Self::Type;
}
impl Zero for f64 {
    type Type = Self;
    const ZERO: Self::Type = 0.0;
}

pub trait One {
    type Type;
    const ONE: Self::Type;
}
impl One for f64 {
    type Type = Self;
    const ONE: Self::Type = 1.0;
}

pub trait Two {
    type Type;
    const TWO: Self::Type;
}
impl Two for f64 {
    type Type = Self;
    const TWO: Self::Type = 2.0;
}

#[macro_export]
macro_rules! impl_zero_one_two_for {
    ($name:ident $($generics:tt)*) => {
        macro_rules! callback {
            ([$$([$$($$decl:tt)+])*] [$$($$use:tt)*]) => {
                impl< $$($$($$decl)+),* > $$crate::Zero for $name< $$($$use)* > {
                    type Type = $$crate::StaticZero<Self>;

                    const ZERO: Self::Type = $$crate::StaticZero::new();
                }

                impl< $$($$($$decl)+),* > $$crate::One for $name< $$($$use)* > {
                    type Type = $$crate::StaticOne<Self>;

                    const ONE: Self::Type = $$crate::StaticOne::new();
                }

                impl< $$($$($$decl)+),* > $$crate::Two for $name< $$($$use)* > {
                    type Type = $$crate::StaticTwo<Self>;

                    const TWO: Self::Type = $$crate::StaticTwo::new();
                }
            };
        }

        $crate::__parse_generics!([$($generics)*] callback!);
    };
}
