use std::ops::DerefMut;

use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::{Gt, Lt, Paren};
use syn::visit_mut::{VisitMut, visit_expr_mut};
use syn::{
    AngleBracketedGenericArguments, Block, Expr, ExprCall, ExprPath, FieldsUnnamed, FnArg,
    GenericArgument, Generics, ItemFn, Pat, Path, ReturnType, Signature, Stmt, Token, Type,
    TypeTuple, parse_macro_input, parse_str, parse2,
};

#[proc_macro_attribute]
pub fn tailcall(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);
    transform(item_fn).to_token_stream().into()
}

const CONTROL_RETURN_TYPE_NAME: &str = "__tailcall_Return";

macro_rules! parse_str2 {
    ($($arg:tt)*) => {
        parse_str(&format!($($arg)*)).unwrap()
    };
}

macro_rules! quote2 {
    ($($e:tt)*) => {
        parse2(quote!($($e)*)).unwrap()
    };
}

fn transform(item_fn: ItemFn) -> ItemFn {
    let outer_function_sig = get_outer_function_signature(&item_fn.sig);

    let args_names = get_args_names(&item_fn.sig);

    let continue_fields_types_names = get_continue_fields_types_names(&args_names);
    let control_generics = get_control_generics(&continue_fields_types_names);
    let continue_fields: FieldsUnnamed =
        parse_str2!("({})", continue_fields_types_names.join(", "));
    let return_field_type = format_ident!("{CONTROL_RETURN_TYPE_NAME}");

    let inner_function_ident = format_ident!("__tailcall_{}", item_fn.sig.ident);
    let inner_function_call: ExprCall =
        parse_str2!("{}({})", inner_function_ident, args_names.join(", "));

    let control_continue_pattern: Pat =
        parse_str2!("__tailcall::Control::Continue({})", args_names.join(", "));

    let inner_function_inputs = item_fn.sig.inputs.clone();

    let outer_function_return_type_generics = get_outer_function_return_type_generics(&item_fn.sig);

    let mut inner_function_body = *item_fn.block;
    handle_control_points(&mut inner_function_body, &item_fn.sig.ident.to_string());
    handle_implicit_return(&mut inner_function_body);

    let mut outer_function: ItemFn = quote2! {
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
                    #control_continue_pattern => control = #inner_function_call,
                    __tailcall::Control::Return(r) => return r,
                }
            }

            fn #inner_function_ident( #inner_function_inputs ) -> __tailcall::Control #outer_function_return_type_generics
                #inner_function_body
        }
    };

    outer_function.attrs = item_fn.attrs;
    outer_function.vis = item_fn.vis;

    outer_function
}

fn get_outer_function_signature(signature: &Signature) -> Signature {
    let mut sig = signature.clone();

    for fn_arg in &mut sig.inputs {
        if let FnArg::Typed(typed) = fn_arg
            && let Pat::Ident(pat_ident) = &mut *typed.pat
        {
            pat_ident.mutability = None;
        }
    }

    sig
}

#[expect(clippy::match_wildcard_for_single_variants)]
fn get_args_names(signature: &Signature) -> Vec<String> {
    signature
        .inputs
        .iter()
        .map(|fn_arg| match fn_arg {
            FnArg::Typed(pat_type) => match &*pat_type.pat {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => todo!("at {}:{}", file!(), line!()),
            },
            _ => todo!("at {}:{}: {fn_arg:?}", file!(), line!()),
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
    parse_str2!("<{}>", types_names.join(", "))
}

fn get_outer_function_return_type_generics(sig: &Signature) -> AngleBracketedGenericArguments {
    let mut outer_function_return_type_generics = AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Lt::default(),
        args: Punctuated::default(),
        gt_token: Gt::default(),
    };

    let mut add_type = |ty: Type| {
        outer_function_return_type_generics
            .args
            .push(GenericArgument::Type(ty));
    };

    for fn_arg in &sig.inputs {
        match fn_arg {
            FnArg::Typed(pat_type) => add_type(pat_type.ty.as_ref().clone()),
            FnArg::Receiver(_) => todo!("at {}:{}", file!(), line!()),
        }
    }

    match &sig.output {
        ReturnType::Type(_, r#type) => add_type(r#type.as_ref().clone()),
        ReturnType::Default => add_type(Type::Tuple(TypeTuple {
            elems: Punctuated::default(),
            paren_token: Paren::default(),
        })),
    }

    outer_function_return_type_generics
}

struct RecursiveCall<'a> {
    path: &'a mut Path,
    args: &'a mut Punctuated<Expr, Token![,]>,
}
impl<'a> RecursiveCall<'a> {
    fn from(expr_call: &'a mut ExprCall, function_name: &str) -> Option<Self> {
        if let Expr::Path(ExprPath { path, .. }) = &mut *expr_call.func
            && path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.first().unwrap().ident == function_name
        {
            return Some(RecursiveCall {
                path,
                args: &mut expr_call.args,
            });
        }

        None
    }
}

fn handle_control_points(block: &mut Block, function_name: &str) {
    struct Visitor<'a> {
        function_name: &'a str,
    }
    impl Visitor<'_> {
        fn turn_into_control_continue(&mut self, recursive_call: &mut RecursiveCall) {
            *recursive_call.path = parse_str2!("__tailcall::Control::Continue");
            for arg in &mut recursive_call.args.iter_mut() {
                self.visit_expr_mut(arg);
            }
        }

        fn turn_into_control_return(&mut self, expr: &mut Option<Box<Expr>>) {
            let mut expr_call: ExprCall = parse_str2!("__tailcall::Control::Return()");

            if let Some(result) = expr {
                let mut arg = result.deref_mut().clone();
                self.visit_expr_mut(&mut arg);
                expr_call.args.push(arg);
            }

            *expr = Some(Box::new(Expr::Call(expr_call)));
        }
    }
    impl VisitMut for Visitor<'_> {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            match expr {
                Expr::Return(expr_return) => {
                    let recursive_call =
                        expr_return.expr.as_deref_mut().and_then(|expr| match expr {
                            Expr::Call(expr_call) => {
                                RecursiveCall::from(expr_call, self.function_name)
                            }
                            _ => None,
                        });

                    if let Some(mut recursive_call) = recursive_call {
                        self.turn_into_control_continue(&mut recursive_call);
                    } else {
                        self.turn_into_control_return(&mut expr_return.expr);
                    }
                }
                Expr::Call(expr_call) => {
                    if let Some(mut recursive_call) =
                        RecursiveCall::from(expr_call, self.function_name)
                    {
                        self.turn_into_control_continue(&mut recursive_call);

                        *expr = quote2! {
                            { return #expr_call; }
                        };
                    } else {
                        visit_expr_mut(self, expr);
                    }
                }
                _ => visit_expr_mut(self, expr),
            }
        }
    }

    let mut visitor = Visitor { function_name };
    visitor.visit_block_mut(block);
}

fn handle_implicit_return(block: &mut Block) {
    if let Some(Stmt::Expr(_)) = block.stmts.last() {
        *block = quote2! {
            {
                let __tailcall_result = #block;
                __tailcall::Control::Return(__tailcall_result)
            }
        };
    }
}
