use std::fs;

use anyhow::Result;
use prettyplease::unparse;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::parse2;

const MAX_ARITY: usize = 5;

fn main() -> Result<()> {
    let items = (1..=MAX_ARITY).flat_map(|arity| {
        let trait_generator = TraitGenerator { arity };

        [
            trait_generator.trait_definition(),
            trait_generator.blanket_implementation(),
        ]
    });
    let tokens = quote! {
        #(#items)*
    };

    fs::write("src/lib.rs", unparse(&parse2(tokens)?))?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TraitGenerator {
    arity: usize,
}
impl TraitGenerator {
    fn trait_definition(self) -> TokenStream {
        let trait_name = self.name();

        let all_types1 = self.types();
        let all_types2 = self.types();

        let methods = (1..=self.arity).flat_map(|binds| {
            let method_generator = MethodGenerator {
                trait_generator: self,
                binds,
            };

            [
                method_generator.bind_method(),
                method_generator.bind_clone_method(),
            ]
        });

        quote! {
            pub trait #trait_name < #(#all_types1),* , R>: FnOnce(#(#all_types2),*) -> R {
                #(#methods)*
            }
        }
    }

    fn blanket_implementation(self) -> TokenStream {
        let trait_name = self.name();

        let types1 = self.types();
        let types2 = self.types();
        let types3 = self.types();

        quote! {
            impl<F, #(#types1),* , R> #trait_name < #(#types2),* , R> for F
            where
                F: FnOnce( #(#types3),* ) -> R,
            {}
        }
    }

    fn name(self) -> Ident {
        format_ident!("Bind{}", self.arity)
    }

    fn types(self) -> impl Iterator<Item = Ident> {
        self.arguments(ArgumentKind::Type)
    }

    fn arguments(self, kind: ArgumentKind) -> impl Iterator<Item = Ident> {
        (1..=self.arity).map(move |n| kind.to_ident(n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MethodGenerator {
    trait_generator: TraitGenerator,
    binds: usize,
}
impl MethodGenerator {
    fn bind_method(self) -> TokenStream {
        let method_name = format_ident!("bind{}", self.binds);

        let types_taken = self.types().taken();
        let vars_taken = self.variables().taken();
        let remaining_types = self.types().remaining();
        let remaining_vars = self.variables().remaining();
        let all_vars = self.variables().all();

        quote! {
            #[must_use]
            fn #method_name (self, #(#vars_taken: #types_taken),* ) -> impl FnOnce( #(#remaining_types),* ) -> R
            where
                Self: Sized,
            {
                move | #(#remaining_vars),* | self( #(#all_vars),* )
            }
        }
    }

    fn bind_clone_method(self) -> TokenStream {
        let method_name = format_ident!("bind{}_clone", self.binds);

        let types_taken1 = self.types().taken();
        let types_taken2 = self.types().taken();
        let vars_taken1 = self.variables().taken();
        let vars_taken2 = self.variables().taken();
        let remaining_types = self.types().remaining();
        let remaining_vars = self.variables().remaining();
        let all_vars = self.variables().all();

        quote! {
            #[must_use]
            fn #method_name (self, #(#vars_taken1: #types_taken1),* ) -> impl Fn( #(#remaining_types),* ) -> R + Clone
            where
                Self: Clone,
                #(#types_taken2: Clone,)*
            {
                move | #(#remaining_vars),* | {
                    let f = self.clone();
                    #(let #vars_taken2 = #vars_taken2.clone();)*
                    f( #(#all_vars),* )
                }
            }
        }
    }

    fn types(self) -> IdentLister {
        self.ident_lister(ArgumentKind::Type)
    }

    fn variables(self) -> IdentLister {
        self.ident_lister(ArgumentKind::Variable)
    }

    fn ident_lister(self, arg_kind: ArgumentKind) -> IdentLister {
        IdentLister {
            trait_generator: self.trait_generator,
            arg_kind,
            binds: self.binds,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IdentLister {
    trait_generator: TraitGenerator,
    arg_kind: ArgumentKind,
    binds: usize,
}
impl IdentLister {
    fn all(self) -> impl Iterator<Item = Ident> {
        self.trait_generator.arguments(self.arg_kind)
    }

    fn taken(self) -> impl Iterator<Item = Ident> {
        self.all().take(self.binds)
    }

    fn remaining(self) -> impl Iterator<Item = Ident> {
        self.all().skip(self.binds)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ArgumentKind {
    Type,
    Variable,
}
impl ArgumentKind {
    fn to_ident(self, arg: usize) -> Ident {
        match self {
            ArgumentKind::Type => format_ident!("A{arg}"),
            ArgumentKind::Variable => format_ident!("a{arg}"),
        }
    }
}
