mod implementations;
mod options;

use proc_macro::TokenStream;
use syn::{Error, ItemStruct, parse_macro_input};

#[proc_macro_derive(Function)]
pub fn function(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::function(&struct_).into()
}

#[proc_macro_derive(Output, attributes(function))]
pub fn output(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::output(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Apply, attributes(function))]
pub fn apply(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::apply(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Derivative, attributes(function))]
pub fn derivative(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::derivative(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(RenameVariable, attributes(function))]
pub fn rename_variable(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::rename_variable(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(VariableNames, attributes(function))]
pub fn variable_names(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::variable_names(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(SetNamedConstValue, attributes(function))]
pub fn set_named_const_value(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::set_named_const_value(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Arity, attributes(function))]
pub fn arity(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::arity(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(ComparisonTokens, attributes(function))]
pub fn comparison_tokens(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::comparison_tokens(&struct_)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Display, attributes(function))]
pub fn display(item: TokenStream) -> TokenStream {
    let struct_ = parse_macro_input!(item as ItemStruct);

    implementations::display(&struct_).into()
}
