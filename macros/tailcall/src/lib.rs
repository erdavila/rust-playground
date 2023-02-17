use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

use crate::dump::dump_item_fn;

mod dump;

#[proc_macro_attribute]
pub fn tailcall(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);

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
