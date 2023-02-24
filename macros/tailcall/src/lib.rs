use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, ItemFn};

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

fn transform(_item_fn: ItemFn) -> ItemFn {
    let outer_function_tokens = quote! {
        // TODO: take signature from parameter
        fn general_recursive(n: u8, trace: Vec<&str>) -> Vec<&str> {
            mod __tailcall {
                // TODO: use arguments names
                pub enum Control<N, Trace, __tailcall_Return> {
                    // TODO: use arguments names
                    Continue(N, Trace),
                    Return(__tailcall_Return),
                }
            }

            // TODO: derive function name from parameter
            // TODO: use arguments names
            let mut control = __tailcall_general_recursive(n, trace);
            loop {
                match control {
                    // TODO: derive function name from parameter
                    // TODO: use arguments names
                    __tailcall::Control::Continue(n, trace) => control = __tailcall_general_recursive(n, trace),
                    __tailcall::Control::Return(r) => return r,
                }
            }

            // TODO: derive function name from parameter
            // TODO: change return type
            fn __tailcall_general_recursive(n: u8, trace: Vec<&str>) -> __tailcall::Control<u8, Vec<&str>, Vec<&str>> {
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
