use convert_case::ccase;
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::{
    Attribute, Error, Fields, Ident, ItemEnum, LitStr, Meta, Path, Result, Token, Variant,
    Visibility, parse_quote,
};

use crate::indexed_struct::context::attribute_option::{
    ExprExt, ListAttrOption, OptionalAttrOption, lit_str_from_name_value_meta,
};

pub(crate) struct Context<'a> {
    pub(super) vis: &'a Visibility,
    pub(super) enum_ident: &'a Ident,
    pub(super) struct_ident: Ident,
    pub(super) fields: Vec<Field>,
    pub(super) type_arg: Ident,
    pub(super) type_arg_2: Ident,
    pub(super) err_type_arg: Ident,
    pub(super) additional_derives: Vec<TokenStream>,
    module: Path,
}

impl<'a> Context<'a> {
    pub(crate) fn new(item_enum: &'a ItemEnum) -> Result<Self> {
        let opts = EnumOptions::parse(&item_enum.attrs)?;

        let vis = &item_enum.vis;
        let enum_ident = &item_enum.ident;
        let struct_ident = if let Some(name) = opts.struct_name.take() {
            Ident::new(&name.value(), name.span())
        } else {
            format_ident!("{}Indexed", item_enum.ident)
        };

        let fields = Field::parse_all(&item_enum.variants, enum_ident)?;

        let (type_arg, type_arg_2) = if enum_ident == "T" || enum_ident == "U" {
            (format_ident!("A"), format_ident!("B"))
        } else {
            (format_ident!("T"), format_ident!("U"))
        };

        let err_type_arg = if enum_ident == "E" {
            format_ident!("X")
        } else {
            format_ident!("E")
        };

        let crate_name: Ident = opts.crate_name.take().map_or_else(
            || parse_quote! { enum_indexed },
            |name| Ident::new(&name.value(), name.span()),
        );

        let module: Path = parse_quote! { ::#crate_name::indexed_struct };
        let additional_derives = opts.derives.take();

        let context = Context {
            vis,
            enum_ident,
            struct_ident,
            fields,
            type_arg,
            type_arg_2,
            err_type_arg,
            additional_derives,
            module,
        };

        Ok(context)
    }

    pub(super) fn trait_path(&self) -> Path {
        let module = &self.module;
        parse_quote! { #module::EnumIndexed }
    }

    pub(super) fn iter_module(&self) -> Path {
        let module = &self.module;
        parse_quote! { #module::iter }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Field {
    pub(super) ident: Ident,
    pub(super) variant: Path,
    pub(super) attrs: Vec<Meta>,
}

impl Field {
    fn parse_all(
        variants: &Punctuated<Variant, Token![,]>,
        enum_ident: &Ident,
    ) -> Result<Vec<Self>> {
        let mut fields = Vec::with_capacity(variants.len());

        for variant in variants {
            if variant.fields != Fields::Unit {
                return Err(Error::new(
                    variant.span(),
                    "Only unit variants are supported by IndexedStruct",
                ));
            }

            let opts = VariantOptions::parse(&variant.attrs)?;

            if let Some((true, span)) = opts.skip.into_option() {
                if opts.field_name.take().is_some() || !opts.attrs.take().is_empty() {
                    return Err(Error::new(
                        span,
                        "The `skip` option cannot be used with other options",
                    ));
                }
            } else {
                let ident = if let Some(name) = opts.field_name.take() {
                    Ident::new(&name.value(), name.span())
                } else {
                    let name = ccase!(snake, variant.ident.to_string());
                    Ident::new(&name, variant.ident.span())
                };

                let variant = {
                    let ident = &variant.ident;
                    parse_quote! { #enum_ident::#ident }
                };

                let attrs = opts.attrs.take().into_iter().flatten().collect();

                fields.push(Field {
                    ident,
                    variant,
                    attrs,
                });
            }
        }

        Ok(fields)
    }
}

#[derive(Debug, Clone, Default)]
struct EnumOptions {
    struct_name: OptionalAttrOption<LitStr>,
    derives: ListAttrOption<TokenStream>,
    crate_name: OptionalAttrOption<LitStr>,
}

impl EnumOptions {
    fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut opts = EnumOptions::default();

        attribute_option::parse(attrs, |opt_processor| {
            opt_processor.accept("name", &mut opts.struct_name, lit_str_from_name_value_meta)?;

            opt_processor.accept("derive", &mut opts.derives, |meta| {
                let list = meta.require_list()?;
                Ok(list.tokens.clone())
            })?;

            opt_processor.accept(
                "enum_indexed_crate",
                &mut opts.crate_name,
                lit_str_from_name_value_meta,
            )?;

            opt_processor.reject_others()
        })?;

        Ok(opts)
    }
}

#[derive(Debug, Clone, Default)]
struct VariantOptions {
    skip: OptionalAttrOption<bool>,
    field_name: OptionalAttrOption<LitStr>,
    attrs: ListAttrOption<Vec<Meta>>,
}

impl VariantOptions {
    fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut opts = VariantOptions::default();

        attribute_option::parse(attrs, |opt_processor| {
            opt_processor.accept("skip", &mut opts.skip, |meta| match meta {
                Meta::Path(_) => Ok(true),
                Meta::NameValue(name_value) => Ok(name_value.value.require_lit_bool()?.value),
                Meta::List(list) => Err(Error::new(list.delimiter.span().open(), "unexpected '('")),
            })?;

            opt_processor.accept("field", &mut opts.field_name, lit_str_from_name_value_meta)?;

            opt_processor.accept("attr", &mut opts.attrs, |meta| {
                let list = meta.require_list()?;
                let attr_metas =
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                let attr_metas: Vec<_> = attr_metas.into_iter().collect();
                Ok(attr_metas)
            })?;

            opt_processor.reject_others()
        })?;

        Ok(opts)
    }
}

mod attribute_option;
