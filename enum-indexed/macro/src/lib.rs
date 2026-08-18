use proc_macro::TokenStream;
use syn::{ItemEnum, parse_macro_input};

mod indexed_struct;

#[proc_macro_derive(IndexedStruct, attributes(indexed_struct))]
pub fn indexed_struct(item: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(item as ItemEnum);

    let token_stream = match indexed_struct::Context::new(&item_enum) {
        Ok(ctx) => indexed_struct::generate(&ctx),
        Err(e) => e.into_compile_error(),
    };

    token_stream.into()
}
