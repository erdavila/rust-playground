use std::fmt::Display;

use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::token::Comma;
use syn::{
    BinOp, Error, Fields, GenericArgument, GenericParam, Ident, Index, ItemStruct, LitInt, Member,
    Result, Token, Type, custom_keyword, parse_quote,
};

custom_keyword!(operands);
custom_keyword!(operator);
custom_keyword!(type_function);

pub(crate) enum Options {
    Const(ConstOptions),
    Operands(OperandsOptions),
}
impl Options {
    pub(crate) fn from_struct(struct_: &ItemStruct) -> Result<Self> {
        match struct_
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("function"))
        {
            Some(attr) => attr.parse_args(),
            None => Err(Error::new(struct_.span(), "attribute `function` not found")),
        }
    }

    pub(crate) fn require_const(&self, trait_name: &str) -> Result<&ConstOptions> {
        match self {
            Options::Const(const_options) => Ok(const_options),
            Options::Operands(operands_options) => Err(option_required_error::<Token![const]>(
                operands_options.operands_token.span,
                trait_name,
            )),
        }
    }

    pub(crate) fn require_operands(&self, trait_name: &str) -> Result<&OperandsOptions> {
        match self {
            Options::Const(const_options) => Err(option_required_error::<operands>(
                const_options.const_token.span,
                trait_name,
            )),
            Options::Operands(operands_options) => Ok(operands_options),
        }
    }

    fn set_option<O: Default + ToTokens, T>(
        opt: &mut Option<T>,
        value: impl FnOnce() -> T,
        span: Span,
    ) -> Result<()> {
        if opt.is_some() {
            Err(Error::new(
                span,
                format_args!(
                    "the `{}` option can only be used once",
                    token_display::<O>()
                ),
            ))
        } else {
            *opt = Some(value());
            Ok(())
        }
    }
}
impl Parse for Options {
    fn parse(input: ParseStream) -> Result<Self> {
        let opts = Punctuated::<DeriveOption, Token![,]>::parse_separated_nonempty(input)?;

        let mut is_const = None;
        let mut type_function = None;
        let mut operands_count = None;
        let mut operator = None;

        for opt in opts {
            match opt {
                DeriveOption::Const { const_token } => {
                    Self::set_option::<Token![const], _>(
                        &mut is_const,
                        || const_token,
                        const_token.span,
                    )?;
                }
                DeriveOption::TypeFunction {
                    type_function_token,
                    ty,
                    ..
                } => {
                    Self::set_option::<type_function, _>(
                        &mut type_function,
                        || ty,
                        type_function_token.span,
                    )?;
                }
                DeriveOption::Operands {
                    operands_token,
                    count,
                    ..
                } => {
                    let count = count.base10_parse::<usize>()?;
                    Self::set_option::<operands, _>(
                        &mut operands_count,
                        || (operands_token, count),
                        operands_token.span,
                    )?;
                }
                DeriveOption::Operator {
                    operator_token, op, ..
                } => {
                    Self::set_option::<operator, _>(&mut operator, || op, operator_token.span)?;
                }
            }
        }

        match (is_const, operands_count) {
            (Some(const_token), Some(_)) => Err(
                conflicting_options_error::<Token![const], operands>(const_token.span),
            ),
            (Some(const_token), None) => {
                if let Some(operator) = operator {
                    return Err(conflicting_options_error::<Token![const], operator>(
                        operator.span(),
                    ));
                }

                let Some(type_function) = type_function else {
                    return Err(Error::new(
                        const_token.span(),
                        format_args!(
                            "the `{}` option is required when `const` is used",
                            token_display::<type_function>()
                        ),
                    ));
                };

                Ok(Options::Const(ConstOptions {
                    const_token,
                    type_function,
                }))
            }
            (None, Some((operands_token, count))) => {
                if let Some(op) = operator
                    && count != 2
                {
                    return Err(Error::new(
                        op.span(),
                        format_args!(
                            "the `{}` option is only supported with `{}` = 2",
                            token_display::<operator>(),
                            token_display::<operands>(),
                        ),
                    ));
                }

                if let Some(type_function) = type_function {
                    return Err(conflicting_options_error::<operands, type_function>(
                        type_function.span(),
                    ));
                }

                Ok(Options::Operands(OperandsOptions {
                    operands_token,
                    count,
                    operator,
                }))
            }
            (None, None) => {
                let lookahead = input.lookahead1();
                lookahead.peek(Token![const]);
                lookahead.peek(operands);
                Err(lookahead.error())
            }
        }
    }
}

pub(crate) struct OperandsOptions {
    pub(crate) operands_token: operands,
    pub(crate) count: usize,
    pub(crate) operator: Option<BinOp>,
}
impl OperandsOptions {
    pub(crate) fn names_and_types(&self, struct_: &ItemStruct) -> (Vec<Member>, Vec<Type>) {
        match &struct_.fields {
            Fields::Named(named) => named
                .named
                .iter()
                .take(self.count)
                .map(|field| {
                    let member = Member::Named(field.ident.as_ref().unwrap().clone());
                    let ty = field.ty.clone();
                    (member, ty)
                })
                .collect(),
            Fields::Unnamed(unnamed) => unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    let member = Member::Unnamed(Index::from(i));
                    let ty = field.ty.clone();
                    (member, ty)
                })
                .collect(),
            Fields::Unit => (Vec::new(), Vec::new()),
        }
    }

    pub(crate) fn generic_args_tail(
        &self,
        generic_args: &Punctuated<GenericArgument, Token![,]>,
    ) -> Punctuated<GenericArgument, Token![,]> {
        generic_args.iter().skip(self.count).cloned().collect()
    }

    pub(crate) fn require_operator(&self, trait_name: &str) -> Result<BinOp> {
        self.operator.ok_or_else(|| {
            Error::new(
                self.operands_token.span,
                format_args!(
                    "the `operator` option is required to derive the `{trait_name}` trait"
                ),
            )
        })
    }
}

pub(crate) struct ConstOptions {
    pub(crate) const_token: Token![const],
    pub(crate) type_function: Box<Type>,
}

pub(crate) struct QuoteValues {
    pub(crate) name: Ident,
    pub(crate) generic_params: Punctuated<GenericParam, Token![,]>,
    pub(crate) generic_args: Punctuated<GenericArgument, Token![,]>,
}
impl From<&ItemStruct> for QuoteValues {
    fn from(struct_: &ItemStruct) -> Self {
        let name = struct_.ident.clone();

        let generic_params = struct_.generics.params.clone();

        let generic_args: Punctuated<GenericArgument, Comma> = struct_
            .generics
            .params
            .iter()
            .map(|param| match param {
                GenericParam::Lifetime(lifetime_param) => {
                    GenericArgument::Lifetime(lifetime_param.lifetime.clone())
                }
                GenericParam::Type(type_param) => {
                    let name = &type_param.ident;
                    GenericArgument::Type(parse_quote! { #name })
                }
                GenericParam::Const(const_param) => {
                    let name = &const_param.ident;
                    GenericArgument::Type(parse_quote! { #name })
                }
            })
            .collect();

        QuoteValues {
            name,
            generic_params,
            generic_args,
        }
    }
}

enum DeriveOption {
    Const {
        const_token: Token![const],
    },
    TypeFunction {
        type_function_token: type_function,
        _eq_token: Token![=],
        ty: Box<Type>,
    },
    Operands {
        operands_token: operands,
        _eq_token: Token![=],
        count: LitInt,
    },
    Operator {
        operator_token: operator,
        _eq_token: Token![=],
        op: BinOp,
    },
}
impl Parse for DeriveOption {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![const]) {
            Ok(DeriveOption::Const {
                const_token: input.parse()?,
            })
        } else if lookahead.peek(type_function) {
            Ok(DeriveOption::TypeFunction {
                type_function_token: input.parse()?,
                _eq_token: input.parse()?,
                ty: input.parse()?,
            })
        } else if lookahead.peek(operands) {
            Ok(DeriveOption::Operands {
                operands_token: input.parse()?,
                _eq_token: input.parse()?,
                count: input.parse()?,
            })
        } else if lookahead.peek(operator) {
            Ok(DeriveOption::Operator {
                operator_token: input.parse()?,
                _eq_token: input.parse()?,
                op: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

fn option_required_error<O: Default + ToTokens>(span: Span, trait_name: &str) -> Error {
    Error::new(
        span,
        format_args!(
            "the `{}` option is required to derive the `{trait_name}` trait",
            token_display::<O>()
        ),
    )
}

fn conflicting_options_error<O1, O2>(span: Span) -> Error
where
    O1: Default + ToTokens,
    O2: Default + ToTokens,
{
    Error::new(
        span,
        format_args!(
            "the `{}` and `{}` options can not be used together",
            token_display::<O1>(),
            token_display::<O2>(),
        ),
    )
}

#[must_use]
fn token_display<T: Default + ToTokens>() -> impl Display {
    T::default().into_token_stream()
}
