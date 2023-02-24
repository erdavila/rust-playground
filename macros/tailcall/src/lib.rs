use std::ops::Deref;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse2, parse_macro_input, parse_str, ExprCall, FieldsUnnamed, FnArg, Generics, ItemFn, Pat,
    Signature,
};

use crate::dump::dump_item_fn;

mod dump;

#[proc_macro_attribute]
pub fn tailcall(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);

    let item_fn = transform(item_fn);

    dump_item_fn(&item_fn);

    TokenStream::from(item_fn.to_token_stream())
}

#[proc_macro_attribute]
pub fn dump(_attr: TokenStream, input: TokenStream) -> TokenStream {
    {
        let input = input.clone();
        let item_fn = parse_macro_input!(input as ItemFn);
        dump_item_fn(&item_fn);
    }

    input
}

const CONTROL_RETURN_TYPE_NAME: &str = "__tailcall_Return";

macro_rules! parse_str2 {
    ($($arg:tt)*) => {
        parse_str(&format!($($arg)*)).unwrap()
    };
}

fn transform(item_fn: ItemFn) -> ItemFn {
    let outer_function_sig = item_fn.sig.clone();

    let args_names = get_args_names(&item_fn.sig);

    let continue_fields_types_names = get_continue_fields_types_names(&args_names);
    let control_generics = get_control_generics(&continue_fields_types_names);
    let continue_fields: FieldsUnnamed =
        parse_str2!("({})", continue_fields_types_names.join(", "));
    let return_field_type = format_ident!("{CONTROL_RETURN_TYPE_NAME}");

    let inner_function_ident = format_ident!("__tailcall_{}", item_fn.sig.ident);
    let inner_function_call: ExprCall =
        parse_str2!("{}({})", inner_function_ident, args_names.join(", "));

    let outer_function_tokens = quote! {
        #outer_function_sig {
            mod __tailcall {
                pub enum Control #control_generics {
                    Continue #continue_fields,
                    Return(#return_field_type),
                }
            }

            let mut control = #inner_function_call;
            loop {
                match control {
                    // TODO: use arguments names
                    __tailcall::Control::Continue(n, trace) => control = #inner_function_call,
                    __tailcall::Control::Return(r) => return r,
                }
            }

            // TODO: change return type
            fn #inner_function_ident(n: u8, trace: Vec<&str>) -> __tailcall::Control<u8, Vec<&str>, Vec<&str>> {
                // TODO: enclose body
                let __tailcall_result = {
                    // TODO: replace returns in body
                    // TODO: replace recursions in body
                    let mut trace = trace;
                    match n {
                        0 => {
                            trace.push("0");
                            return __tailcall::Control::Continue(1, trace);
                        }
                        1 => {
                            trace.push("1");
                            return __tailcall::Control::Continue(2, trace);
                        }
                        2 => {
                            return __tailcall::Control::Continue(3, {
                                trace.push("2");
                                trace
                            })
                        }
                        3 => {
                            return __tailcall::Control::Continue(4, {
                                trace.push("3");
                                trace
                            })
                        }
                        4 => {
                            trace.push("4");
                            trace
                        }
                        _ => {
                            trace.push("_");
                            return __tailcall::Control::Return(trace);
                        }
                    }
                };
                __tailcall::Control::Return(__tailcall_result)
            }
        }
    };

    parse2::<ItemFn>(outer_function_tokens).unwrap()
}

fn get_args_names(signature: &Signature) -> Vec<String> {
    signature
        .inputs
        .iter()
        .map(|fn_arg| match fn_arg {
            FnArg::Typed(pat_type) => match pat_type.pat.deref() {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => todo!(),
            },
            _ => todo!(),
        })
        .collect()
}

fn get_continue_fields_types_names(args_names: &[String]) -> Vec<String> {
    args_names
        .iter()
        .map(|name| snake_case_to_upper_camel_case(name))
        .collect()
}

fn snake_case_to_upper_camel_case(str: &str) -> String {
    str.split('_').map(capitalize).collect()
}

fn capitalize(str: &str) -> String {
    let mut chars = str.chars();

    let first_char = chars.next();

    let mut out = String::with_capacity(str.len());
    if let Some(first_char) = first_char {
        out.extend(first_char.to_uppercase());
    }
    out.extend(chars);

    out
}

fn get_control_generics(continue_fields_types_names: &[String]) -> Generics {
    let mut types_names = continue_fields_types_names.to_owned();
    types_names.push(CONTROL_RETURN_TYPE_NAME.to_string());
    parse_str(&format!("<{}>", types_names.join(", "))).unwrap()
}
