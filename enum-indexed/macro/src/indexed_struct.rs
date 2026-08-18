use proc_macro2::TokenStream;
use quote::quote;

pub(super) use crate::indexed_struct::context::Context;

mod context;

pub(super) fn generate(ctx: &Context) -> TokenStream {
    let struct_ = struct_(ctx);
    let trait_impl = trait_impl(ctx);
    let index_impls = index_impls(ctx);
    let into_iter_impls = into_iter_impls(ctx);

    quote! {
        #struct_
        #trait_impl
        #index_impls
        #into_iter_impls
    }
}

fn struct_(ctx: &Context) -> TokenStream {
    let additional_derives = &ctx.additional_derives;
    let vis = ctx.vis;
    let struct_ident = &ctx.struct_ident;
    let type_arg = &ctx.type_arg;

    let fields = ctx.fields.iter().map(|field| {
        let field_ident = &field.ident;
        let attrs = field.attrs.iter().map(|attr| {
            quote! { #[#attr] }
        });

        quote! {
            #(#attrs)*
            pub #field_ident: #type_arg
        }
    });

    quote! {
        #[automatically_derived]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, #(#additional_derives),*)]
        #vis struct #struct_ident<#type_arg> {
            #(#fields),*
        }
    }
}

fn trait_impl(ctx: &Context) -> TokenStream {
    let type_arg = &ctx.type_arg;
    let type_arg_2 = &ctx.type_arg_2;
    let trait_path = ctx.trait_path();
    let enum_ident = ctx.enum_ident;
    let struct_ident = &ctx.struct_ident;
    let count = ctx.fields.len();

    let try_from_fn_method = try_from_fn_method(ctx);
    let try_map_enumerated_method = try_map_enumerated_method(ctx);
    let as_ref_methods = as_ref_methods(ctx);
    let swap_method = swap_method(ctx);
    let into_array_enumerated_method = into_array_enumerated_method(ctx);
    let variant_from_index_method = variant_from_index_method(ctx);

    quote! {
        #[automatically_derived]
        impl<#type_arg> #trait_path<#type_arg, #count> for #struct_ident<#type_arg> {
            type Enum = #enum_ident;
            type Map<#type_arg_2> = #struct_ident<#type_arg_2>;

            #try_from_fn_method
            #try_map_enumerated_method
            #as_ref_methods
            #swap_method
            #into_array_enumerated_method
            #variant_from_index_method
        }
    }
}

fn try_from_fn_method(ctx: &Context) -> TokenStream {
    let type_arg = &ctx.type_arg;
    let err_type_arg = &ctx.err_type_arg;
    let struct_ident = &ctx.struct_ident;

    let field_inits = ctx.fields.iter().map(|field| {
        let field_ident = &field.ident;
        let variant = &field.variant;
        quote! { #field_ident: f(#variant)? }
    });

    quote! {
        fn try_from_fn<#err_type_arg>(mut f: impl ::core::ops::FnMut(Self::Enum) -> ::core::result::Result<#type_arg, #err_type_arg>) -> ::core::result::Result<Self, #err_type_arg> {
            ::core::result::Result::Ok(#struct_ident {
                #(#field_inits),*
            })
        }
    }
}

fn try_map_enumerated_method(ctx: &Context) -> TokenStream {
    let type_arg = &ctx.type_arg;
    let type_arg_2 = &ctx.type_arg_2;
    let err_type_arg = &ctx.err_type_arg;
    let struct_ident = &ctx.struct_ident;

    let field_inits = ctx.fields.iter().map(|field| {
        let field_ident = &field.ident;
        let variant = &field.variant;
        quote! { #field_ident: f(#variant, self.#field_ident)? }
    });

    quote! {
        fn try_map_enumerated<#type_arg_2, #err_type_arg>(
            self,
            mut f: impl ::core::ops::FnMut(Self::Enum, #type_arg) -> ::core::result::Result<#type_arg_2, #err_type_arg>,
        ) -> ::core::result::Result<Self::Map<#type_arg_2>, #err_type_arg> {
            ::core::result::Result::Ok(#struct_ident {
                #(#field_inits),*
            })
        }
    }
}

fn as_ref_methods(ctx: &Context) -> TokenStream {
    let type_arg = &ctx.type_arg;
    let struct_ident = &ctx.struct_ident;

    let field_refs = ctx.fields.iter().map(|field| {
        let field_ident = &field.ident;
        quote! { #field_ident: &self.#field_ident }
    });

    let field_mut_refs = ctx.fields.iter().map(|field| {
        let field_ident = &field.ident;
        quote! { #field_ident: &mut self.#field_ident }
    });

    quote! {
        fn as_ref(&self) -> Self::Map<&#type_arg> {
            #struct_ident {
                #(#field_refs),*
            }
        }

        fn as_mut(&mut self) -> Self::Map<&mut #type_arg> {
            #struct_ident {
                #(#field_mut_refs),*
            }
        }
    }
}

fn swap_method(ctx: &Context) -> TokenStream {
    let pairs = ctx
        .fields
        .iter()
        .flat_map(|field_x| ctx.fields.iter().map(move |field_y| (field_x, field_y)));

    let arms = pairs.map(|(field_x, field_y)| {
        let field_ident_x = &field_x.ident;
        let field_ident_y = &field_y.ident;
        let variant_x = &field_x.variant;
        let variant_y = &field_y.variant;

        let body = if field_x == field_y {
            quote! { () }
        } else {
            quote! { ::core::mem::swap(&mut self.#field_ident_x, &mut self.#field_ident_y) }
        };

        quote! {
            (#variant_x, #variant_y) => #body
        }
    });

    quote! {
        fn swap(&mut self, x: Self::Enum, y: Self::Enum) {
            match (x, y) {
                #(#arms,)*
                _ => ::core::panic!("variant is not supported as an index"),
            }
        }
    }
}

fn into_array_enumerated_method(ctx: &Context) -> TokenStream {
    let type_arg = &ctx.type_arg;
    let count = ctx.fields.len();
    let elements = ctx.fields.iter().map(|field| {
        let variant = &field.variant;
        let field_ident = &field.ident;
        quote! { (#variant, self.#field_ident) }
    });

    quote! {
        fn into_array_enumerated(self) -> [(Self::Enum, #type_arg); #count] {
            [ #(#elements),* ]
        }
    }
}

fn variant_from_index_method(ctx: &Context) -> TokenStream {
    let arms = ctx.fields.iter().enumerate().map(|(index, field)| {
        let variant = &field.variant;
        quote! { #index => #variant }
    });

    quote! {
        fn variant_from_index(index: usize) -> Self::Enum {
            match index {
                #(#arms,)*
                _ => ::core::panic!("invalid index"),
            }
        }
    }
}

fn index_impls(ctx: &Context) -> TokenStream {
    let type_arg = &ctx.type_arg;
    let enum_ident = ctx.enum_ident;
    let struct_ident = &ctx.struct_ident;

    let ref_arms = ctx.fields.iter().map(|field| {
        let variant = &field.variant;
        let field_ident = &field.ident;
        quote! { #variant => &self.#field_ident }
    });

    let mut_arms = ctx.fields.iter().map(|field| {
        let variant = &field.variant;
        let field_ident = &field.ident;
        quote! { #variant => &mut self.#field_ident }
    });

    quote! {
        #[automatically_derived]
        impl<#type_arg> ::core::ops::Index<#enum_ident> for #struct_ident<#type_arg> {
            type Output = #type_arg;

            fn index(&self, index: #enum_ident) -> &Self::Output {
                match index {
                    #(#ref_arms,)*
                    _ => ::core::panic!("variant is not supported as an index"),
                }
            }
        }

        #[automatically_derived]
        impl<#type_arg> ::core::ops::IndexMut<#enum_ident> for #struct_ident<#type_arg> {
            fn index_mut(&mut self, index: #enum_ident) -> &mut Self::Output {
                match index {
                    #(#mut_arms,)*
                    _ => ::core::panic!("variant is not supported as an index"),
                }
            }
        }
    }
}

fn into_iter_impls(ctx: &Context) -> TokenStream {
    let struct_ident = &ctx.struct_ident;
    let enum_ident = ctx.enum_ident;
    let type_arg = &ctx.type_arg;
    let iter_module = ctx.iter_module();
    let count = ctx.fields.len();

    quote! {
        #[automatically_derived]
        impl<#type_arg> ::core::iter::IntoIterator for #struct_ident<#type_arg> {
            type Item = (#enum_ident, #type_arg);

            type IntoIter = #iter_module::IntoIter<#enum_ident, #type_arg, #count>;

            fn into_iter(self) -> Self::IntoIter {
                #iter_module::IntoIter::new(self)
            }
        }

        #[automatically_derived]
        impl<'a, #type_arg> ::core::iter::IntoIterator for &'a #struct_ident<#type_arg> {
            type Item = (#enum_ident, &'a #type_arg);

            type IntoIter = #iter_module::Iter<'a, #type_arg, #count, #struct_ident<#type_arg>>;

            fn into_iter(self) -> Self::IntoIter {
                #iter_module::Iter::new(self)
            }
        }

        #[automatically_derived]
        impl<'a, #type_arg> ::core::iter::IntoIterator for &'a mut #struct_ident<#type_arg> {
            type Item = (#enum_ident, &'a mut #type_arg);

            type IntoIter = #iter_module::IterMut<'a, #type_arg, #count, #struct_ident<#type_arg>>;

            fn into_iter(self) -> Self::IntoIter {
                #iter_module::IterMut::new(self)
            }
        }
    }
}
