use std::collections::VecDeque;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Expr, ItemStruct, Result, parse_quote};

use crate::options::{ConstOptions, Options, QuoteValues};

pub(crate) fn function(struct_: &ItemStruct) -> TokenStream2 {
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    quote! {
        #[automatically_derived]
        impl < #generic_params > ::symbolic_calculus::Function for #name < #generic_args > {}
    }
}

pub(crate) fn output(struct_: &ItemStruct) -> Result<TokenStream2> {
    let options = Options::from_struct(struct_)?;
    let operands_options = options.require_operands("Output")?;

    let op = operands_options.require_operator("Output")?;
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);
    let (_, operand_types) = operands_options.names_and_types(struct_);

    let left_ty = &operand_types[0];
    let right_ty = &operand_types[1];

    Ok(quote! {
        #[automatically_derived]
        #[ops_bounds(
            #left_ty: ::symbolic_calculus::Output<I>,
            #right_ty: ::symbolic_calculus::Output<I>,
            #left_ty::Output #op #right_ty::Output,
        )]
        impl < #generic_params, I> ::symbolic_calculus::Output<I> for #name < #generic_args > {
            type Output = ops_type!(#left_ty::Output #op #right_ty::Output);
        }
    })
}

pub(crate) fn apply(struct_: &ItemStruct) -> Result<TokenStream2> {
    let options = Options::from_struct(struct_)?;
    let operands_options = options.require_operands("Output")?;

    let op = operands_options.require_operator("Output")?;
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);
    let (operand_names, operand_types) = operands_options.names_and_types(struct_);

    let left_name = &operand_names[0];
    let right_name = &operand_names[1];

    let left_ty = &operand_types[0];
    let right_ty = &operand_types[1];

    Ok(quote! {
        #[automatically_derived]
        #[ops_bounds(
            #left_ty: ::symbolic_calculus::Apply<I>,
            #right_ty: ::symbolic_calculus::Apply<I>,
            #left_ty::Output #op #right_ty::Output,
            I: Clone,
        )]
        impl < #generic_params, I> ::symbolic_calculus::Apply<I> for #name < #generic_args > {
            fn apply(this: &Self, input: I) -> Self::Output {
                let l = this.#left_name.apply(input.clone());
                let r = this.#right_name.apply(input);
                l #op r
            }
        }
    })
}

pub(crate) fn derivative(struct_: &ItemStruct) -> Result<TokenStream2> {
    let options = Options::from_struct(struct_)?;
    let ConstOptions { type_function, .. } = options.require_const("Derivative")?;

    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    Ok(quote! {
        #[automatically_derived]
        impl < #generic_params, const V: usize> ::symbolic_calculus::Derivative<V> for #name < #generic_args > {
            type Derivative = StaticZero<#type_function>;

            fn derivative(_this: &Self) -> Self::Derivative {
                StaticZero::new()
            }
        }
    })
}

pub(crate) fn rename_variable(struct_: &ItemStruct) -> Result<TokenStream2> {
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    match Options::from_struct(struct_)? {
        Options::Const(_) => Ok(quote! {
            #[automatically_derived]
            impl < #generic_params, const V: usize> ::symbolic_calculus::RenameVariable<V> for #name < #generic_args >
            where
                Self: Clone,
            {
                type RenamedVar<N> = Self;

                fn rename_var_num<N: Clone>(this: &Self, _name: N) -> Self::RenamedVar<N> {
                    this.clone()
                }
            }
        }),
        Options::Operands(operands_options) => {
            let (operand_names, operand_types) = operands_options.names_and_types(struct_);
            let generic_args_tail = operands_options.generic_args_tail(&generic_args);

            Ok(quote! {
                #[automatically_derived]
                impl < #generic_params, const V: usize> ::symbolic_calculus::RenameVariable<V> for #name < #generic_args >
                where
                    #(#operand_types: ::symbolic_calculus::RenameVariable<V>,)*
                {
                    type RenamedVar<N> = #name <
                        #(#operand_types::RenamedVar<N>,)*
                        #generic_args_tail
                    >;

                    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
                        #name::new(
                            #(this.#operand_names.rename_var_num(name.clone()),)*
                        )
                    }
                }
            })
        }
    }
}

pub(crate) fn variable_names(struct_: &ItemStruct) -> Result<TokenStream2> {
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    match Options::from_struct(struct_)? {
        Options::Const(_) => Ok(quote! {
            #[automatically_derived]
            impl < #generic_params > ::symbolic_calculus::VariableNames for #name < #generic_args > {
                fn var_names(&self) -> impl Iterator<Item = (usize, &dyn ::core::fmt::Display)> {
                    ::core::iter::empty()
                }
            }
        }),
        Options::Operands(operands_options) => {
            let (operand_names, operand_types) = operands_options.names_and_types(struct_);

            let mut iter_exprs: VecDeque<Expr> = operand_names
                .into_iter()
                .map(|name| {
                    parse_quote! { self.#name.var_names() }
                })
                .collect();

            // Merge the iterators 2 by 2.
            let iter_expr = loop {
                let iter1 = iter_exprs.pop_front().unwrap();
                let Some(iter2) = iter_exprs.pop_front() else {
                    break iter1;
                };

                iter_exprs.push_back(
                    parse_quote! { ::symbolic_calculus::util::MergedVarNames::new(#iter1, #iter2) },
                );
            };

            Ok(quote! {
                #[automatically_derived]
                impl < #generic_params > ::symbolic_calculus::VariableNames for #name < #generic_args >
                where
                    #(#operand_types: ::symbolic_calculus::VariableNames,)*
                {
                    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn ::core::fmt::Display)> {
                        #iter_expr
                    }
                }
            })
        }
    }
}

pub(crate) fn set_named_const_value(struct_: &ItemStruct) -> Result<TokenStream2> {
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    match Options::from_struct(struct_)? {
        Options::Const(_) => Ok(quote! {
            #[automatically_derived]
            impl < #generic_params, Id> ::symbolic_calculus::SetNamedConstValue<Id> for #name < #generic_args >
            where
                Self: Clone,
            {
                type NamedConstValueSet<V> = Self;

                fn set_named_const_value<V: ::symbolic_calculus::ValuedConst + Clone>(
                    this: &Self,
                    _value: V,
                ) -> Self::NamedConstValueSet<V> {
                    this.clone()
                }
            }
        }),
        Options::Operands(operands_options) => {
            let (operand_names, operand_types) = operands_options.names_and_types(struct_);
            let generic_args_tail = operands_options.generic_args_tail(&generic_args);

            Ok(quote! {
                #[automatically_derived]
                impl < #generic_params, Id> ::symbolic_calculus::SetNamedConstValue<Id> for #name < #generic_args >
                where
                    #(#operand_types: ::symbolic_calculus::SetNamedConstValue<Id>,)*
                    Id: ::symbolic_calculus::NamedConstId,
                {
                    type NamedConstValueSet<V> = #name <
                        #(#operand_types::NamedConstValueSet<V>,)*
                        #generic_args_tail
                    >;

                    fn set_named_const_value<V: ::symbolic_calculus::ValuedConst + Clone>(
                        this: &Self,
                        value: V,
                    ) -> Self::NamedConstValueSet<V> {
                        #name::new(
                            #(this.#operand_names.set_named_const_value(value.clone()),)*
                        )
                    }
                }
            })
        }
    }
}

pub(crate) fn arity(struct_: &ItemStruct) -> Result<TokenStream2> {
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    match Options::from_struct(struct_)? {
        Options::Const(_) => Ok(quote! {
            #[automatically_derived]
            impl < #generic_params > ::symbolic_calculus::Arity for #name < #generic_args > {
                fn arity(&self) -> usize {
                    0
                }
            }
        }),
        Options::Operands(operands_options) => {
            let (operand_names, operand_types) = operands_options.names_and_types(struct_);

            Ok(quote! {
                #[automatically_derived]
                impl < #generic_params > ::symbolic_calculus::Arity for #name < #generic_args >
                where
                    #(#operand_types: ::symbolic_calculus::Arity,)*
                {
                    fn arity(&self) -> usize {
                        [
                            #(self.#operand_names.arity(),)*
                        ].into_iter().max().unwrap()
                    }
                }
            })
        }
    }
}

pub(crate) fn comparison_tokens(struct_: &ItemStruct) -> Result<TokenStream2> {
    let options = Options::from_struct(struct_)?;
    let operands_options = options.require_operands("ComparisonTokens")?;

    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    let (operand_names, operand_types) = operands_options.names_and_types(struct_);

    Ok(quote! {
        #[automatically_derived]
        impl < #generic_params > ::symbolic_calculus::ComparisonTokens for #name < #generic_args >
        where
            #(#operand_types: ::symbolic_calculus::ComparisonTokens,)*
        {
            fn comparison_tokens(
                &self,
            ) -> impl Iterator<Item = ::symbolic_calculus::comparison::Token<'_>> {
                let id = ::symbolic_calculus::comparison_id!(#name);
                ::symbolic_calculus::comparison::TokensBuilder::new(id)
                    #(.from_child(&self.#operand_names))*
                    .build()
            }
        }
    })
}

pub(crate) fn display(struct_: &ItemStruct) -> TokenStream2 {
    let QuoteValues {
        name,
        generic_params,
        generic_args,
    } = QuoteValues::from(struct_);

    quote! {
        #[automatically_derived]
        impl < #generic_params > ::core::fmt::Display for #name < #generic_args >
        where
            Self: ::symbolic_calculus::FunctionDisplay,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.display_fmt(f)
            }
        }
    }
}
