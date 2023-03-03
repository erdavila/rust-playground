use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Error, Item, Lit, Meta, NestedMeta, Result};

mod indentation;
mod tokens;
mod tree;

#[proc_macro_attribute]
pub fn dump(attr: TokenStream, input: TokenStream) -> TokenStream {
    {
        let args = parse_macro_input!(attr as AttributeArgs);
        let args = match Args::from(args) {
            Ok(args) => args,
            Err(err) => return err.into_compile_error().into(),
        };

        let input = input.clone();
        let item = parse_macro_input!(input as Item);

        let file_path = match args.file_path_or_from(&item) {
            Ok(file_path) => file_path,
            Err(err) => return err.into_compile_error().into(),
        };

        if args.tree {
            tree::dump_item(&item, &(file_path.clone() + ".tree"));
        }
        if args.tokens {
            tokens::dump_item(&item, &(file_path + ".tokens"));
        }
    }

    input
}

#[derive(Debug)]
struct Args {
    name: Option<String>,
    prefix: String,
    suffix: String,
    path: Option<String>,
    tokens: bool,
    tree: bool,
}

impl Args {
    fn from(attr_args: AttributeArgs) -> Result<Self> {
        let mut args = Args {
            name: None,
            prefix: "".to_string(),
            suffix: "".to_string(),
            path: None,
            tokens: true,
            tree: true,
        };

        for attr_arg in attr_args {
            if let Some((key, value)) = Self::key_value_from(&attr_arg)? {
                match &key[..] {
                    "name" => args.name = Some(Self::str_value_from(value)?),
                    "prefix" => args.prefix = Self::str_value_from(value)?,
                    "suffix" => args.suffix = Self::str_value_from(value)?,
                    "path" => args.path = Some(Self::str_value_from(value)?),
                    "tokens" => args.tokens = Self::bool_value_from(value)?,
                    "tree" => args.tree = Self::bool_value_from(value)?,
                    _ => return Err(Error::new_spanned(attr_arg, "Unrecognized argument")),
                }
            } else {
                todo!("at {}:{}", file!(), line!())
            }
        }

        Ok(args)
    }

    fn key_value_from(nested_meta: &NestedMeta) -> Result<Option<(String, &Lit)>> {
        if let NestedMeta::Meta(Meta::NameValue(meta_name_value)) = nested_meta {
            if let Some(key) = meta_name_value.path.get_ident() {
                Ok(Some((key.to_string(), &meta_name_value.lit)))
            } else {
                Err(Error::new_spanned(nested_meta, "Unsupported argument"))
            }
        } else {
            Ok(None)
        }
    }

    fn str_value_from(lit: &Lit) -> Result<String> {
        if let Lit::Str(lit_str) = lit {
            Ok(lit_str.value())
        } else {
            Err(Error::new(lit.span(), "Expected str argument"))
        }
    }

    fn bool_value_from(lit: &Lit) -> Result<bool> {
        if let Lit::Bool(lit_bool) = lit {
            Ok(lit_bool.value)
        } else {
            Err(Error::new(lit.span(), "Expected bool argument"))
        }
    }

    fn file_path_or_from(&self, item: &Item) -> Result<String> {
        let mut file_path = String::new();

        if let Some(path) = &self.path {
            file_path.push_str(path);
            file_path.push('/');
        }

        let name_base = if let Some(name) = &self.name {
            name.to_owned()
        } else if let Some(name) = Self::name_from_item(item) {
            name
        } else {
            return Err(Error::new_spanned(
                item,
                "Can't deduce item name for #[dump]",
            ));
        };

        file_path.push_str(&format!("{}{}{}", self.prefix, name_base, self.suffix));

        Ok(file_path)
    }

    fn name_from_item(item: &Item) -> Option<String> {
        let name = match item {
            Item::Const(item_const) => item_const.ident.to_string(),
            Item::Enum(item_enum) => item_enum.ident.to_string(),
            Item::ExternCrate(item_extern_crate) => item_extern_crate.ident.to_string(),
            Item::Fn(item_fn) => item_fn.sig.ident.to_string(),
            Item::Macro(item_macro) => {
                if let Some(ident) = &item_macro.ident {
                    ident.to_string()
                } else {
                    return None;
                }
            }
            Item::Macro2(item_macro2) => item_macro2.ident.to_string(),
            Item::Mod(item_mod) => item_mod.ident.to_string(),
            Item::Static(item_static) => item_static.ident.to_string(),
            Item::Struct(item_struct) => item_struct.ident.to_string(),
            Item::Trait(item_trait) => item_trait.ident.to_string(),
            Item::TraitAlias(item_trait_alias) => item_trait_alias.ident.to_string(),
            Item::Type(item_type) => item_type.ident.to_string(),
            Item::Union(item_union) => item_union.ident.to_string(),
            _ => return None,
        };

        Some(name)
    }
}
